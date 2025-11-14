use clap::{Arg, Command};

fn main() -> anyhow::Result<()> {
    let command = Command::new("Bazel updater application")
        .version("0.1")
        .author("Jose Luis Ojosnegros Manchon <jl.ojosnegros.manchon@gmail.com>")
        .about("Simple application to create/update dependencies in bazel repository_locations.bzl files")
        .arg(
            Arg::new("repo_loc")
               .short('f')
               .long("repo_loc_file")
               .help("repository_locations.bzl file location"),
        );

    let matches = command.get_matches();

    Ok(())
}
