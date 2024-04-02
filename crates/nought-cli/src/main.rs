use clap::error::ErrorKind::Io;
use clap::Parser;
use crate::commands::create::create;

use crate::commands_config::{Args, Commands};
use crate::errors::build_error;

mod commands;
mod commands_config;
mod errors;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.commands {
        Commands::Create { platform, dir_name, version } => create(platform, dir_name, version)
                .await
                .unwrap_or_else(|err|
                    build_error(Io, err.to_string()).exit()),
        Commands::Install { plugin_name } => {}
    }
}