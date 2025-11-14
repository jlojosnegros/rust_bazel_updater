use std::collections::HashMap;

use anyhow::Ok;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dependency {
    // =================
    // Mandatory fields
    // =================
    pub project_name: String,
    pub project_desc: String,
    pub project_url: String,
    pub version: String,
    pub sha256: String,

    /// URLs to download dependency
    pub urls: Vec<String>,

    // =================
    // Optional fields
    // =================
    /// Prefix to remove when extracting archive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip_prefix: Option<String>,

    /// Release Date (Format: YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_category: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,

    /// Common Platform Enumeration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpe: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub implied_untracked_deps: Option<Vec<String>>,
}

pub type RepositoryLocations = HashMap<String, Dependency>;

impl Dependency {
    pub fn new(
        project_name: String,
        project_desc: String,
        project_url: String,
        version: String,
        sha256: String,
        urls: Vec<String>,
    ) -> Self {
        Self {
            project_name: project_name,
            project_desc: project_desc,
            project_url: project_url,
            version: version,
            sha256: sha256,
            urls: urls,
            strip_prefix: None,
            release_date: None,
            use_category: None,
            license: None,
            license_url: None,
            cpe: None,
            implied_untracked_deps: None,
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        let mut errors = Vec::new();

        if self.project_name.is_empty() {
            errors.push("project_name cannot be empty");
        }
        if self.project_desc.is_empty() {
            errors.push("project_desc cannot be empty");
        }
        if self.project_url.is_empty() {
            errors.push("project_url cannot be empty");
        }
        if self.version.is_empty() {
            errors.push("version cannot be empty");
        }
        if self.sha256.len() != 64 {
            errors.push("sh256 must be 64 characteres (256 bits in hex)");
        }
        if self.urls.is_empty() {
            errors.push("urls cannot be empty");
        }

        if errors.is_empty() {
            Ok(())
        } else {
            let error_msg = format!(
                "Validation failed for '{}' @ '{}:\n  - {}",
                self.project_name,
                self.version,
                errors.join("\n  - ")
            );
            anyhow::bail!(error_msg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_creation() {
        let dep = Dependency::new(
            "test-project".to_string(),
            "A test project".to_string(),
            "https://example.com".to_string(),
            "1.0.0".to_string(),
            "a".repeat(64), // SHA256 válido (64 chars hex)
            vec!["https://example.com/download".to_string()],
        );

        assert_eq!(dep.project_name, "test-project");
        assert_eq!(dep.version, "1.0.0");
        assert!(dep.license.is_none());
    }

    #[test]
    fn test_validation_success() {
        let dep = Dependency::new(
            "test".to_string(),
            "desc".to_string(),
            "url".to_string(),
            "1.0".to_string(),
            "a".repeat(64),
            vec!["https://url".to_string()],
        );
        assert!(dep.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_sha256() {
        let mut dep = Dependency::new(
            "test".to_string(),
            "desc".to_string(),
            "url".to_string(),
            "1.0".to_string(),
            "short".to_string(), // ❌ Demasiado corto
            vec!["url".to_string()],
        );
        assert!(dep.validate().is_err());
    }
}
