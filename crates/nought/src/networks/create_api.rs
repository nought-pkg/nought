use std::env;
use std::error::Error;
use std::fmt::Write;

use colored::Colorize;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use reqwest::StatusCode;
use tokio::fs::{create_dir, File, OpenOptions};
use tokio::io::{AsyncWriteExt, BufWriter};

use crate::errors::common_errors::NetworkError;
use crate::errors::create_errors::{ConfigSerializeFailed, CreateServerDirectoryFailed, VersionNotFound};
use crate::proto::config_proto::Config;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub async fn download_spigot_core(path: String, version: String) -> Result<()> {
    let download_url = format!("https://download.getbukkit.org/spigot/spigot-{}.jar", version);
    println!("{} {}", "Downloading ".green().bold(), download_url);
    let response = reqwest::get(download_url)
        .await
        .map_err(|_| Box::new(NetworkError))?;

    if response.status() != StatusCode::OK {
        return Err(Box::new(VersionNotFound))
    }
    let content_length = response.content_length().unwrap_or(0);
    let mut stream = response.bytes_stream();
    let mut file = File::create(format!("{}/spigot-{}.jar", path, version))
        .await?;

    let pb = ProgressBar::new(content_length);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk).await?;
        pb.inc(chunk.len() as u64);
    }

    file.flush().await?;

    pb.finish();

    Ok(())
}

pub async fn create_config(path: String, name: String) -> Result<()> {
    println!("{} {}/{}/.nought", "Creating".green().bold(), env::current_dir()?.display(), name);
    create_dir(format!("{}/.nought", path))
        .await
        .map_err(|_| Box::new(CreateServerDirectoryFailed))?;
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}/.nought/config.toml", path))
        .await?;
    println!("{} {}", "Creating".green().bold(),
             format!("{}/{}/.nought/config.toml", env::current_dir()?.display(), name));
    let config = toml::to_string(&Config {
        name
    }).map_err(|_| Box::new(ConfigSerializeFailed))?;
    let mut writer = BufWriter::new(file);
    writer.write(config.as_bytes()).await?;
    writer.flush().await?;
    println!("{}", "Finished initializing!".green().bold());
    Ok(())
}
