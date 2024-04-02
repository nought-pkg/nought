use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
bin_name = "nought",
author = env!("CARGO_PKG_AUTHORS"),
version = env!("CARGO_PKG_VERSION"),
about = env!("CARGO_PKG_DESCRIPTION"),
long_about = None,
)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) commands: Commands
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Create a server from a platform
    Create {
        platform: String,
        dir_name: String
    },
    /// Install a plugin to a server
    Install {
        plugin_name: String,
    }
}
