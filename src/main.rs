use std::str::FromStr;

use clap::{AppSettings, Parser, Subcommand};
use errors::TackleError;
use log::{error, LevelFilter};

use crate::cli::{initialize, install, list};

mod cache;
mod cli;
mod conditions;
mod errors;
mod package;
mod project;
mod util;

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
    Install {
        /// The URL of the hook to install.
        url: String,
    },
    /// Uninstall a git hook from the target repository.
    /// This will remove the hook from the repository and remove the hook file from the hooks directory.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Uninstall { package: String },
    /// List all installed hooks.
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

fn main() {
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
        Install { url } => install(url),
        List => list(),
        _ => todo!(),
    };
    // run the error handler on error
    if let Err(e) = res {
        error!("{}", e);
    }
}
