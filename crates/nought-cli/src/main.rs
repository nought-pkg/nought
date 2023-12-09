mod commands;
mod commands_config;
mod errors;

use crate::commands_config::{Args, Commands};
use crate::errors::common_errors::repos_file_read_error;
use clap::{Error, Parser};
use tokio::fs::OpenOptions;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    let api_sources = read_repos().await.unwrap_or_else(|error| error.exit());
    let args = Args::parse();

    match args.commands {
        Commands::Install { pkg_name } => commands::install(pkg_name),
        Commands::Sync => commands::sync(),
        Commands::Upgrade { pkg_name } => commands::upgrade(pkg_name),
        Commands::Search { pkg_name } => commands::search(api_sources, &pkg_name)
            .await.unwrap_or_else(|error| error.exit()),
    }
}

async fn read_repos() -> Result<Vec<String>, Error> {
    let repos = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("repos")
        .await
        .map_err(|_| repos_file_read_error())?;
    let mut api_sources = Vec::new();
    let mut lines = BufReader::new(repos).lines();
    while let Some(line) = lines.next_line().await.map_err(|_| repos_file_read_error())? {
        api_sources.push(line)
    }
    Ok(api_sources)
}
