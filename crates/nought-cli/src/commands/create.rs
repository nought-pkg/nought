use std::str::FromStr;

use clap::Error;
use clap::error::ErrorKind::Io;

use nought::networks::create_api::{create_config, download_spigot_core};

use crate::errors::build_error;

pub(crate) async fn create(platform: String, dir_name: String, version: String) {
    tokio::fs::create_dir(format!("./{}", dir_name))
        .await
        .unwrap_or_else(|err| build_error(Io, err.to_string()).exit());

    let platform = platform.parse::<Platform>()
        .unwrap_or_else(|err| err.exit());

    match platform {
        Platform::Spigot => {
            download_spigot_core(format!("./{}", dir_name), version)
                .await
                .unwrap_or_else(|err| build_error(Io, err.to_string()).exit());
            create_config(format!("./{}", dir_name), dir_name)
                .await
                .unwrap_or_else(|err| build_error(Io, err.to_string()).exit())
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
