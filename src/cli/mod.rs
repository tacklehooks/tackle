//! Contains the CLI logic and commands.

mod initialize;
mod install;
mod list;

use std::str::FromStr;

use clap::{AppSettings, Parser, Subcommand};

pub use initialize::*;
pub use install::*;
pub use list::*;
use log::{error, LevelFilter};

use crate::errors::TackleError;

/// Multi-platform, agnostic git hook manager.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
    /// Enable extra debug logging.
    #[clap(long, short, global = true)]
    debug: bool,
}

enum Hook {
    PreCommit,
}

impl FromStr for Hook {
    type Err = TackleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "precommit" => Ok(Hook::PreCommit),
            _ => Err(TackleError::InvalidCommitHook),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Install a git hook from the target repository.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    #[clap(alias = "install", alias = "i")]
    Add {
        /// The URL of the hook to add.
        url: String,
    },
    /// Remove a git hook from the target repository.
    /// This will remove the hook from the repository and remove the hook file from the hooks directory.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    #[clap(alias = "rm", alias = "uninstall", alias = "ui")]
    Remove { package: String },
    /// List all installed hooks.
    #[clap(alias = "ls", alias = "l")]
    List,
    /// Query the hook repository for a list of available hooks.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Query {
        /// The name of the package to query.
        package: String,
    },
    /// Initialize this project with Tackle.
    #[clap(alias = "init")]
    Initialize,
}

#[tracing::instrument]
pub fn run_cli() {
    let args = Args::parse();
    // if debug mode is enabled, initialize a more verbose logger.
    if args.debug {
        env_logger::builder()
            .filter_level(LevelFilter::Debug)
            .init();
    } else {
        pretty_env_logger::formatted_builder()
            .filter_level(LevelFilter::Info)
            .init();
    }
    // match subcommand
    use Commands::*;
    let res = match args.command {
        Initialize => initialize(),
        Add { url } => install(url),
        List => list(),
        _ => todo!(),
    };
    // run the error handler on error
    if let Err(e) = res {
        error!("{}", e);
    }
}
