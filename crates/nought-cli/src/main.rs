use clap::{Parser, Subcommand};

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Install { .. } => {}
        Commands::Sync => {}
        Commands::Upgrade { .. } => {}
        Commands::Search { .. } => {}
    }
}

#[derive(Parser)]
#[command(
    bin_name = "nought",
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = None,
)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
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
