use log::info;

use crate::{
    errors::TackleError,
    project::{check_tackle_directory_exists, create_tackle_directory},
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
