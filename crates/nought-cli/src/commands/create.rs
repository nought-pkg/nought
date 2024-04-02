use std::str::FromStr;

use clap::Error;

use nought::networks::create_api::{create_config, download_spigot_core};

use crate::errors::create_errors::{create_config_error, create_core_file_error, dir_already_exist_error, unsupported_platform_error};

pub(crate) async fn create(platform: String, dir_name: String) {
    tokio::fs::create_dir(format!("./{}", dir_name))
        .await
        .unwrap_or_else(|_| dir_already_exist_error().exit());

    let platform = platform.parse::<Platform>()
        .unwrap_or_else(|err| err.exit());

    match platform {
        Platform::Spigot => {
            download_spigot_core(format!("./{}", dir_name), "1.20.4".to_string())
                .await
                .unwrap_or_else(|_| create_core_file_error().exit());
            create_config(format!("./{}", dir_name), dir_name)
                .await
                .unwrap_or_else(|_| create_config_error().exit())
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
            _ => Err(unsupported_platform_error())
        }
    }
}
