use clap::{command, Arg, Command};

pub fn build_cli() -> Command {
    command!()
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(Command::new("sync").about("Syncs the local database"))
        .subcommand(
            Command::new("query")
                .arg(Arg::new("pattern"))
                .about("queries based on local databases"),
        )
}
