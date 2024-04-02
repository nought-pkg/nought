use std::str::FromStr;

use clap::Error;
use clap::error::ErrorKind::Io;

use nought::networks::create_api::{create_config, download_spigot_core};
use crate::errors::build_error;

pub(crate) async fn create(platform: String, dir_name: String, version: String) -> nought::networks::Result<()> {
    tokio::fs::create_dir(format!("./{}", dir_name))
        .await?;

    let platform = platform.parse::<Platform>()?;

    match platform {
        Platform::Spigot => {
            download_spigot_core(format!("./{}", dir_name), version)
                .await?;
            create_config(format!("./{}", dir_name), dir_name)
                .await?;
            Ok(())
        }
    }
}

enum Platform {
    Spigot,
}

impl FromStr for Platform {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "spigot" => Ok(Platform::Spigot),
            _ => Err(build_error(Io, "Unsupported platform.".to_string()))
        }
    }
}
