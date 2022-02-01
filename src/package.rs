//! Contains definitions for interacting with a Tackle package.
use std::fs;

use git2::Repository;
use lazy_static::lazy_static;
use log::debug;
use regex::Regex;
use serde::Deserialize;

use crate::{errors::TackleError, project::get_project_root};

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

lazy_static! {
    static ref URL_REGEX: Regex =
        Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9-]{1,61}[a-zA-Z0-9]\.[a-zA-Z]{2,}").unwrap();
}

/// Attempt to clone the repository at the given URL into the hook directory.
pub fn fetch_package<S: AsRef<str>>(package: S) -> Result<Package, TackleError> {
    let package = package.as_ref();
    // package must have at least one slash
    debug!("Validating package URL...");
    if !package.contains('/') {
        panic!("invalid package name");
    }
    debug!("Fetching package '{}'...", package);
    // if package does not contain a tld, then assume it is a github repo
    let tld = package.split('/').next().unwrap();
    let repo_url = match URL_REGEX.is_match(tld) {
        true => package.to_owned(),
        false => format!("github.com/{}", package),
    };

    let repo_url = repo_url.split("/").take(3).collect::<Vec<_>>().join("/");

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_url_regex() {
        let url = "github.com/rust-lang/rust";
        assert!(super::URL_REGEX.is_match(url));
        let url = "rust-lang/rust";
        assert!(!super::URL_REGEX.is_match(url));
    }
}
