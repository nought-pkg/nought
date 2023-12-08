mod commands;
mod errors;
mod commands_config;

use clap::Parser;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncBufReadExt, BufReader};
use crate::commands_config::{Args, Commands};
use crate::errors::common_errors::repos_file_read_error;

#[tokio::main]
async fn main() {
    let api_sources = read_repos().await;
    let args = Args::parse();

    match args.commands {
        Commands::Install { pkg_name } => commands::install(pkg_name),
        Commands::Sync => commands::sync(),
        Commands::Upgrade { pkg_name } => commands::upgrade(pkg_name),
        Commands::Search { pkg_name } => commands::search(api_sources, &pkg_name).await
    }
}

async fn read_repos() -> Vec<String> {
    let repos = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("repos")
        .await
        .unwrap_or_else(|_| repos_file_read_error().exit());
    let mut api_sources = Vec::new();
    let mut lines = BufReader::new(repos).lines();
    while let Some(line) = lines.next_line().await.unwrap_or_else(|_| repos_file_read_error().exit()) {
        api_sources.push(line)
    }
    api_sources
}