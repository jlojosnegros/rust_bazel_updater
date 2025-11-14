use clap::{ArgMatches, Command};

pub fn configure() -> Command {
    Command::new("create").about("Create a new dependency entry in repository_location file")
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("create") {
        println!("TBD");
    }

    Ok(())
}
