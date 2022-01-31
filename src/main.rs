use std::{path::PathBuf, str::FromStr};

use clap::{AppSettings, Parser, Subcommand};
use errors::TackleError;
use log::{error, info, LevelFilter};
use manifest::read_manifest;

use crate::{
    manifest::{
        create_tackle_directory, tackle_directory_exists, write_manifest, TackleManifest,
        TackleManifestHook, TackleManifestHooks,
    },
    packages::fetch_package,
};

mod errors;
mod manifest;
mod packages;

/// Multi-platform, agnostic git hook manager.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
    /// Enable extra debug logging.
    #[clap(long, short)]
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
        /// The type of hook to install.
        hook: Hook,
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

fn get_project_root() -> Result<PathBuf, TackleError> {
    let cwd = std::env::current_dir().unwrap();
    let repo =
        git2::Repository::discover(&cwd).map_err(|_err| TackleError::RepositoryDiscoveryFailed)?;
    // check repoistory work directory exists
    if let None = repo.workdir() {
        return Err(TackleError::RepositoryDiscoveryFailed);
    }
    Ok(repo.workdir().unwrap().to_owned())
}

/// Initialize a new project.
fn initialize() -> Result<(), TackleError> {
    let cwd = std::env::current_dir().unwrap();
    let repo =
        git2::Repository::discover(&cwd).map_err(|_err| TackleError::RepositoryDiscoveryFailed)?;
    // check repoistory work directory exists
    if let None = repo.workdir() {
        return Err(TackleError::RepositoryDiscoveryFailed);
    }
    // check if tackle directory exists
    let workdir = repo.workdir().unwrap();
    if tackle_directory_exists(workdir) {
        return Err(TackleError::AlreadyInitialized);
    }
    // create tackle directory
    info!("Initializing a new project with Tackle...");
    create_tackle_directory(workdir)?;
    Ok(())
}

/// Install a git hook from the target repository.
fn install(hook: Hook, url: String) -> Result<(), TackleError> {
    let workdir = get_project_root()?;
    let manifest = read_manifest(&workdir)?;
    // extract hook array reference
    let hooks = match hook {
        Hook::PreCommit => &manifest.hooks.precommit,
    };
    // check to see if the hook is already installed
    if let Some(hooks) = hooks {
        let exists = hooks.iter().find(|hook| hook.url == url).is_some();
        if exists {
            return Err(TackleError::AlreadyInitialized);
        }
    }
    info!("Installing '{}'...", url);
    let package = fetch_package(&url)?;

    // create the manifest hook entry
    let manifest_hook = TackleManifestHook {
        url: url.clone(),
        integrity: "".to_owned(),
        version: "1".to_owned(),
    };
    // create the new manifest file
    let append_hook = |mut hooks: Vec<TackleManifestHook>| {
        hooks.push(manifest_hook.clone());
        hooks
    };
    let new_manifest = match hook {
        Hook::PreCommit => TackleManifest {
            hooks: TackleManifestHooks {
                precommit: Some(
                    manifest
                        .hooks
                        .precommit
                        .map_or(vec![manifest_hook.clone()], append_hook),
                ),
                ..manifest.hooks
            },
            ..manifest
        },
    };
    // write the new manifest
    write_manifest(&workdir, &new_manifest)?;

    Ok(())
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
        Install { hook, url } => install(hook, url),
        _ => todo!(),
    };

    if let Err(e) = res {
        error!("{}", e);
    }
}
