use std::fs;

use git2::Repository;
use lazy_static::lazy_static;
use log::debug;
use regex::Regex;
use serde::Deserialize;

use crate::{errors::TackleError, get_project_root, config::TackleManifestHooks, config::Package};

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
