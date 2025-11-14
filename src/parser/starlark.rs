use std::collections::HashMap;

use starlark::{
    environment::{Globals, Module},
    eval::Evaluator,
    syntax::{AstModule, Dialect},
    values::{Value, dict::DictRef, list::ListRef},
};

use crate::models::{Dependency, RepositoryLocations};

pub struct StarlarkParser;

impl StarlarkParser {
    pub fn parse(content: &str) -> anyhow::Result<RepositoryLocations> {
        let ast = AstModule::parse(
            "repository_locations.bzl",
            content.to_owned(),
            &Dialect::Standard,
        )
        .map_err(|e| anyhow::anyhow!("Failed to parse Starlark file: {}", e))?;

        let globals = Globals::standard();
        let module = Module::new();

        {
            let mut eval = Evaluator::new(&module);

            eval.eval_module(ast, &globals)
                .map_err(|e| anyhow::anyhow!("Failed to evaluate Starlark module: {}", e))?;
            let repo_spec_value = module
                .get("REPOSITORY_LOCATIONS_SPEC")
                .ok_or_else(|| anyhow::anyhow!("REPOSITORY_LOCATIONS_SPEC not found in file"))?;

            Self::value_to_repository_locations(repo_spec_value)
        }
    }

    fn value_to_repository_locations(value: Value) -> anyhow::Result<RepositoryLocations> {
        let mut result = HashMap::new();

        let dict = DictRef::from_value(value)
            .ok_or_else(|| anyhow::anyhow!("REPOSITORY_LOCATIONS_SPEC is not a dict"))?;

        for (key, val) in dict.iter() {
            let dependency_name = key
                .unpack_str()
                .ok_or_else(|| anyhow::anyhow!("Dependency key is not a string"))?;

            let dependency = Self::value_to_dependency(val)?;
            result.insert(dependency_name.to_string(), dependency);
        }

        Ok(result)
    }

    fn value_to_dependency(value: Value) -> anyhow::Result<Dependency> {
        let dict = DictRef::from_value(value)
            .ok_or_else(|| anyhow::anyhow!("Dependency value is not a dict"))?;
        // ---------------------------
        // Some helper functions
        // ---------------------------

        let get_string = |key: &str| -> anyhow::Result<String> {
            dict.get_str(key)
                .ok_or_else(|| anyhow::anyhow!("Missing required field {}", key))?
                .unpack_str()
                .ok_or_else(|| anyhow::anyhow!("Field {} is not a string", key))
                .map(|s| s.to_string())
        };

        let get_optional_string = |key: &str| -> Option<String> {
            dict.get_str(key)
                .and_then(|v| v.unpack_str())
                .map(|s| s.to_string())
        };

        let get_string_vec = |key: &str| -> anyhow::Result<Vec<String>> {
            let list_value = dict
                .get_str(key)
                .ok_or_else(|| anyhow::anyhow!("Missing required field {} ", key))?;

            let list_ref = ListRef::from_value(list_value)
                .ok_or_else(|| anyhow::anyhow!("Field {} is not a list", key))?;

            let mut result = Vec::new();

            for item in list_ref.iter() {
                let s = item
                    .unpack_str()
                    .ok_or_else(|| anyhow::anyhow!("Item in {} is not a string", key))?;
                result.push(s.to_string());
            }

            Ok(result)
        };

        let get_optional_string_vec = |key: &str| -> Option<Vec<String>> {
            dict.get_str(key).and_then(|list_value| {
                let list_ref = ListRef::from_value(list_value)?;
                let mut result = Vec::new();
                for item in list_ref.iter() {
                    let s = item.unpack_str()?;
                    result.push(s.to_string());
                }
                Some(result)
            })
        };
        // ---------------------------
        // End helper functions
        // ---------------------------

        Ok(Dependency {
            project_name: get_string("project_name")?,
            project_desc: get_string("project_desc")?,
            project_url: get_string("project_url")?,
            version: get_string("version")?,
            sha256: get_string("sha256")?,
            urls: get_string_vec("urls")?,
            strip_prefix: get_optional_string("strip_prefix"),
            release_date: get_optional_string("release_date"),
            use_category: get_optional_string_vec("use_category"),
            license: get_optional_string("license"),
            license_url: get_optional_string("license_url"),
            cpe: get_optional_string("cpe"),
            implied_untracked_deps: get_optional_string_vec("implied_untracked_deps"),
        })
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_repository() {
        let content = r#"
REPOSITORY_LOCATIONS_SPEC = dict(
    test_dep = dict(
        project_name = "Test Project",
        project_desc = "A test dependency",
        project_url = "https://example.com",
        version = "1.0.0",
        sha256 = "a" * 64,
        urls = ["https://example.com/download.tar.gz"],
    ),
)
"#;

        let result = StarlarkParser::parse(content);
        assert!(result.is_ok());

        let locations = result.unwrap();
        assert_eq!(locations.len(), 1);
        assert!(locations.contains_key("test_dep"));

        let dep = &locations["test_dep"];
        assert_eq!(dep.project_name, "Test Project");
        assert_eq!(dep.version, "1.0.0");
    }
}

