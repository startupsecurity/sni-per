mod cli;
mod commands;
mod utils;

#[tokio::main]
async fn main() {
    let matches = cli::build_cli().get_matches();

    // Directly await the result of the command execution
    match matches.subcommand() {
        Some(("sync", _)) => {
            commands::sync::run().await; // Ensure commands::sync::run is an async function and await it
        }
        Some(("query", matches)) => {
            let pattern = matches.get_one::<String>("pattern").expect("Pattern is a required argument");
            commands::query::run(pattern).await; 
        }
        _ => {
            eprintln!("Invalid command");
        }
    };
}
