//! Contains types and methods for interacting with a project where Tackle is installed.
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

use lazy_static::lazy_static;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::errors::TackleError;

/// The default manifest file.
pub static DEFAULT_MANIFEST: &str = include_str!("assets/tackle.toml");
/// The default gitignore file.
pub static DEFAULT_GITIGNORE: &str = include_str!("assets/.gitignore");

lazy_static! {
    /// The path to the project root. This is cached to avoid repeated calls to `get_project_root`.
    pub static ref TACKLE_DIR: Mutex<Option<String>> = Mutex::new(None);
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct TackleManifestHook {
    pub url: String,
    pub version: String,
    pub integrity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TackleManifestHooks {
    #[serde(default = "Vec::new")]
    pub precommit: Vec<TackleManifestHook>,
    #[serde(default = "Vec::new")]
    pub postcommit: Vec<TackleManifestHook>,
    #[serde(default = "Vec::new")]
    pub prepush: Vec<TackleManifestHook>,
    #[serde(default = "Vec::new")]
    pub postpush: Vec<TackleManifestHook>,
}

/// The manifest file.
#[derive(Debug, Serialize, Deserialize)]
pub struct TackleManifest {
    /// The manifest version.
    pub version: String,
    /// A list of installed hooks.
    pub hooks: TackleManifestHooks,
}

/// Read the manifest file.
pub fn read_manifest<P: AsRef<Path>>(workdir: P) -> Result<TackleManifest, TackleError> {
    debug!(
        "Reading manifest file at '{}/.tackle/tackle.toml'",
        workdir.as_ref().display()
    );
    let path = workdir.as_ref().join(".tackle/tackle.toml");
    let contents = fs::read_to_string(&path).map_err(|_| TackleError::ManifestReadFailed)?;
    let manifest: TackleManifest =
        toml::from_str(&contents).map_err(TackleError::ManifestParseFailed)?;
    Ok(manifest)
}

pub fn write_manifest<P: AsRef<Path>>(
    workdir: P,
    manifest: &TackleManifest,
) -> Result<(), TackleError> {
    let path = workdir.as_ref().join(".tackle/tackle.toml");
    let contents = toml::to_string(manifest).unwrap();
    fs::write(&path, contents).map_err(|_err| TackleError::ManifestWriteFailed)?;
    Ok(())
}

/// Create the tackle directory if it does not exist.
pub fn create_tackle_directory<P: AsRef<Path>>(workdir: P) -> Result<(), TackleError> {
    let path = workdir.as_ref().join(".tackle");
    if !path.exists() {
        fs::create_dir_all(&path).map_err(TackleError::CreateTackleDirectoryFailed)?;
    }
    // create the empty hooks directory
    let hooks_dir = &path.join("hooks");
    if !hooks_dir.exists() {
        fs::create_dir_all(hooks_dir)
            .map_err(TackleError::CreateTackleDirectoryFailed)?;
    }
    // write the default manifest
    let manifest_path = &path.join("tackle.toml");
    fs::write(manifest_path, DEFAULT_MANIFEST).map_err(|_err| TackleError::ManifestWriteFailed)?;
    // write the default .gitignore
    let gitignore_path = &path.join(".gitignore");
    fs::write(gitignore_path, DEFAULT_GITIGNORE)
        .map_err(|_err| TackleError::ManifestWriteFailed)?;

    Ok(())
}

/// Fetch the project root.
pub fn get_project_root() -> Result<PathBuf, TackleError> {
    debug!("Discovering project root...");
    let cwd = std::env::current_dir().unwrap();
    let repo =
        git2::Repository::discover(&cwd).map_err(|_err| TackleError::RepositoryDiscoveryFailed)?;
    // check repoistory work directory exists
    if let None = repo.workdir() {
        return Err(TackleError::RepositoryDiscoveryFailed);
    }
    let project_root = repo.workdir().unwrap().to_owned();
    debug!("Project root discovered: {}", project_root.display());
    Ok(project_root)
}

/// Test if the project is initialized.
pub fn is_initialized() -> bool {
    debug!("Checking initialization state of project...");
    let project_root = get_project_root();
    if let Err(_) = project_root {
        return false;
    }
    check_tackle_directory_exists(&project_root.unwrap())
}

/// Test if the tackle directory exists.
pub fn check_tackle_directory_exists<P: AsRef<Path>>(workdir: P) -> bool {
    let path = workdir.as_ref().join(".tackle");
    path.exists() && path.is_dir()
}
