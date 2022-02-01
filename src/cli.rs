//! Contains the definitions of CLI subcommands.
use log::info;

use crate::{
    errors::TackleError,
    package::fetch_package,
    project::{
        check_tackle_directory_exists, create_tackle_directory, get_project_root, is_initialized,
        read_manifest, TackleManifestHook,
    },
};

/// Initialize a new project.
pub fn initialize() -> Result<(), TackleError> {
    let cwd = std::env::current_dir().unwrap();
    let repo =
        git2::Repository::discover(&cwd).map_err(|_err| TackleError::RepositoryDiscoveryFailed)?;
    // check repoistory work directory exists
    if let None = repo.workdir() {
        return Err(TackleError::RepositoryDiscoveryFailed);
    }
    // check if tackle directory exists
    let workdir = repo.workdir().unwrap();
    if check_tackle_directory_exists(workdir) {
        return Err(TackleError::AlreadyInitialized);
    }
    // create tackle directory
    info!("Initializing a new project with Tackle...");
    create_tackle_directory(workdir)?;
    Ok(())
}

/// Install a git hook from the target repository.
pub fn install(url: String) -> Result<(), TackleError> {
    if !is_initialized() {
        return Err(TackleError::NotInitialized);
    }
    let workdir = get_project_root()?;
    let manifest = read_manifest(&workdir)?;
    // install package
    info!("Installing '{}'...", url);
    let package = fetch_package(&url)?;
    // create the manifest hook entry
    let manifest_hook = TackleManifestHook {
        url: url.clone(),
        integrity: "".to_owned(),
        version: "1".to_owned(),
    };
    // // create the new manifest file
    // let append_hook = |mut hooks: Vec<TackleManifestHook>| {
    //     hooks.push(manifest_hook.clone());
    //     hooks
    // };
    // let new_manifest = match hook {
    //     Hook::PreCommit => TackleManifest {
    //         hooks: TackleManifestHooks {
    //             precommit: Some(
    //                 manifest
    //                     .hooks
    //                     .precommit
    //                     .map_or(vec![manifest_hook.clone()], append_hook),
    //             ),
    //             ..manifest.hooks
    //         },
    //         ..manifest
    //     },
    // };
    // // write the new manifest
    // write_manifest(&workdir, &new_manifest)?;

    Ok(())
}

pub fn list() -> Result<(), TackleError> {
    if !is_initialized() {
        return Err(TackleError::NotInitialized);
    }

    let workdir = get_project_root()?;
    let manifest = read_manifest(&workdir)?;

    println!("Pre-commit Hooks:");
    for hook in &manifest.hooks.precommit {
        println!("\t{}", hook.url);
    }
    if manifest.hooks.precommit.is_empty() {
        println!("\tNo hooks installed.");
    }
    println!("Post-commit Hooks:");
    for hook in &manifest.hooks.postcommit {
        println!("\t{}", hook.url);
    }
    if manifest.hooks.precommit.is_empty() {
        println!("\tNo hooks installed.");
    }

    Ok(())
}
