use clap::{ArgMatches, Command};

pub fn configure() -> Command {
    Command::new("update").about("Update an existent dependency entry in repository_location file")
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("update") {
        println!("TBD");
    }

    Ok(())
}

