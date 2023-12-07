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
    /// Install a package with package name
    Install {
        /// Package Name
        pkg_name: String
    },
    /// Sync packages index from the source
    Sync,
    /// Upgrade a package
    Upgrade {
        /// Package Name
        pkg_name: Option<String>
    },
    /// Search a package with package name
    Search {
        /// Package Name
        pkg_name: String
    }
}
