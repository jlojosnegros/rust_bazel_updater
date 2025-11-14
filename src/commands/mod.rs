use clap::{ArgMatches, Command};

mod hello;

pub fn configure(command: Command) -> Command {
    command.subcommand(hello::configure())
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    hello::handle(matches)?;

    Ok(())
}
