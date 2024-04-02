use std::fmt::Write;
use colored::Colorize;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn download_spigot_core(path: String, version: String) -> Result<()> {
    println!("{}", "Connecting to spigot...".blue());
    let response = reqwest::get(format!("https://download.getbukkit.org/spigot/spigot-{}.jar", version))
        .await?;
    let content_length = response.content_length().unwrap_or(0);
    let mut stream = response.bytes_stream();
    let mut file = File::create(format!("{}/spigot-{}.jar", path, version)).await?;

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
    Ok(())
}
