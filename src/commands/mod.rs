use clap::{ArgMatches, Command};

mod create;
mod hello;
mod update;

pub fn configure(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(create::configure())
        .subcommand(update::configure())
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    hello::handle(matches)?;
    create::handle(matches)?;
    update::handle(matches)?;

    Ok(())
}
