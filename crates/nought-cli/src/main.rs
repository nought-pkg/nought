use clap::Parser;
use crate::commands::create::create;

use crate::commands_config::{Args, Commands};

mod commands;
mod commands_config;
mod errors;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.commands {
        Commands::Create { platform, dir_name } => create(platform, dir_name).await,
        Commands::Install { plugin_name } => {}
    }
}