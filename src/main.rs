use bazel_updater::commands;
use clap::{Arg, Command};
use dotenv::dotenv;

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let mut command = Command::new("Bazel updater application")
        .version("0.1")
        .author("Jose Luis Ojosnegros Manchon <jl.ojosnegros.manchon@gmail.com>")
        .about("Simple application to create/update dependencies in bazel repository_locations.bzl files")
        .arg(
            Arg::new("repo_loc")
               .short('f')
               .long("repo_loc_file")
               .help("repository_locations.bzl file location"),
        );

    // configure subcommands
    command = commands::configure(command);

    // get matches
    let matches = command.get_matches();

    // handle matches
    commands::handle(&matches)?;

    Ok(())
}
