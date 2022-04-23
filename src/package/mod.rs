//! This module definitions for interacting with a Tackle package, as well as the logic
//! for installing packages.
//!
//! Package installation happens in 3 steps:
//! - Resolution
//! - Download
//! - Link
//!
//! The resolution step is responsible for resolving the packagee name to either a download URL, or a cached
//! package.
//! The download step is responsible for downloading the package from the URL, and caching it.
//! The linking step is responsible for linking the pacakge from the cache to the project.
pub mod cache;
pub mod download;
pub mod link;
pub mod resolve;

use std::{fs, path::Path};

use git2::Repository;
use log::debug;
use serde::Deserialize;

use crate::{errors::TackleError, project::get_project_root, util::package_into_git_url};

/// A `tackle.toml` file defining a hook package.
#[derive(Deserialize)]
pub struct Package {
    /// The name of the package.
    pub name: Option<String>,
    /// A description of the package.
    pub description: Option<String>,
    /// The version of the package.
    pub version: Option<String>,
    /// Hooks defined by this package.
    pub hooks: HookDefinitions,
}

impl Package {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Package, TackleError> {
        let contents = fs::read_to_string(path)?;
        let package: Package = toml::from_str(&contents)?;
        Ok(package)
    }
}

/// A collection of hooks defined by a package.
#[derive(Deserialize)]
pub struct HookDefinitions {
    /// A list of hook definitions for the pre-commit hook.
    #[serde(default = "Vec::new")]
    pub precommit: Vec<HookDefinition>,
    /// A list of hook definitions for the post-commit hook.
    #[serde(default = "Vec::new")]
    pub postcommit: Vec<HookDefinition>,
}

/// A hook definition inside a Tackle package.
#[derive(Deserialize, PartialEq, Debug)]
pub struct HookDefinition {
    /// The ID of the hook. This field is used to identify the hook in
    /// condition blocks of other hooks.
    pub id: Option<String>,
    /// The command to run.
    pub command: Vec<String>,
    /// OS-level dependencies for the hook.
    #[serde(default = "Vec::new")]
    pub dependencies: Vec<String>,
    /// A vector of conditions to test before the hook is run.
    #[serde(default = "Vec::new")]
    pub conditions: Vec<HookCondition>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct HookCondition {
    /// Matches successful tasks.
    #[serde(default = "Vec::new")]
    pub successful: Vec<String>,
    /// Matches failed tasks.
    #[serde(default = "Vec::new")]
    pub failed: Vec<String>,
    /// Matches skipped tasks.
    #[serde(default = "Vec::new")]
    pub skipped: Vec<String>,
    /// Matches files.
    #[serde(default = "Vec::new")]
    pub exists: Vec<String>,
    /// Matches the current branch.
    #[serde(default = "Vec::new")]
    pub branch: Vec<String>,
}

/// Attempt to clone the repository at the given URL into the hook directory.
pub fn fetch_package<S: AsRef<str>>(package: S) -> Result<Package, TackleError> {
    let repo_url = package_into_git_url(package)?;
    debug!("Repository URL: {}", repo_url);
    let path = get_project_root()?.join(".tackle/hooks").join(&repo_url);
    // if the work directory exists, PANIC!!
    debug!("Checking if the package directory exists...");
    if path.exists() {
        panic!("attempting to clone into existing directory");
    }

    // clone the repository
    debug!("Cloning repository...");
    let repository = Repository::clone(&format!("https://{}.git", repo_url), path)
        .map_err(|_| TackleError::RepositoryCloneFailed)?;
    // get the manifest file
    debug!("Reading manifest file...");
    let workdir = match repository.workdir() {
        Some(workdir) => workdir,
        None => panic!("workdir not found"),
    };
    // lookup the manifest path
    let manifest_path = workdir.join("package.toml");
    let manifest_path = match manifest_path.exists() && manifest_path.is_file() {
        true => manifest_path,
        false => panic!("manifest not found"),
    };

    // read and parse the manifest file
    let manifest =
        fs::read_to_string(&manifest_path).map_err(|_| TackleError::ManifestReadFailed)?;

    match toml::from_str::<Package>(&manifest) {
        Ok(manifest) => Ok(manifest),
        Err(err) => panic!("manifest parse error: {}", err),
    }
}
