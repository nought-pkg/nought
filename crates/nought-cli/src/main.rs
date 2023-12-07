mod commands;
mod errors;
mod commands_config;

use clap::{Parser, Subcommand, CommandFactory};
use crate::commands_config::{Args, Commands};

fn main() {
    let args = Args::parse();

    match args.commands {
        Commands::Install { pkg_name } => commands::install(pkg_name),
        Commands::Sync => commands::sync(),
        Commands::Upgrade { pkg_name } => commands::upgrade(pkg_name),
        Commands::Search { pkg_name } => commands::search(pkg_name)
    }
}