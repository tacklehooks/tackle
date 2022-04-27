//! Contains various utilites and useful methods.
use std::{env, fmt::Debug, fs};

use lazy_static::lazy_static;
use regex::Regex;

use crate::errors::TackleError;

/// Test if the target executable exists in path.
#[tracing::instrument]
pub fn is_program_in_path(program: &str) -> bool {
    let delimiter = if cfg!(windows) { ";" } else { ":" };
    if let Ok(path) = env::var("PATH") {
        for p in path.split(delimiter) {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

lazy_static! {
    static ref URL_REGEX: Regex =
        Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9-]{1,61}[a-zA-Z0-9]\.[a-zA-Z]{2,}").unwrap();
}

/// Translate a package name into a valid Git repository URL.
#[tracing::instrument]
pub fn package_into_git_url<S: AsRef<str> + Debug>(url: S) -> Result<String, TackleError> {
    let repo_url = resolve_package_url(url)?;
    let repo_url = repo_url.split("/").take(3).collect::<Vec<_>>().join("/");
    Ok(repo_url)
}

/// Resolve a package name into a valid path.
#[tracing::instrument]
pub fn resolve_package_url<S: AsRef<str> + Debug>(url: S) -> Result<String, TackleError> {
    let url = url.as_ref();
    // package must have at least one slash
    if !url.contains('/') {
        panic!("invalid url name");
    }
    // if url does not contain a tld, then assume it is a github repo
    let tld = url.split('/').next().unwrap();
    let repo_url = match URL_REGEX.is_match(tld) {
        true => url.to_owned(),
        false => format!("github.com/{}", url),
    };
    Ok(repo_url)
}

/// Extracts the relative path from the repository root to a package's directory from its repository URL.
#[tracing::instrument]
pub fn extract_package_path<S: AsRef<str> + Debug>(url: S) -> Result<String, TackleError> {
    let url = resolve_package_url(url)?;
    // skip the git server and repository names
    let last = url.split('/').skip(3).collect::<Vec<_>>().join("/");
    if last == "" {
        Ok(".".to_owned())
    } else {
        Ok(last)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::{is_program_in_path, package_into_git_url};

    #[test]
    fn test_url_regex() {
        let url = "github.com/rust-lang/rust";
        assert!(super::URL_REGEX.is_match(url));
        let url = "rust-lang/rust";
        assert!(!super::URL_REGEX.is_match(url));
    }

    #[test]
    fn test_is_program_in_path() {
        assert!(is_program_in_path("git"));
        assert!(!is_program_in_path("not_a_program"));
    }

    #[test]
    fn test_package_into_git_url() {
        assert_eq!(
            package_into_git_url("skyezerfox/hooks").unwrap(),
            "github.com/skyezerfox/hooks"
        );
        assert_eq!(
            package_into_git_url("github.com/skyezerfox/hooks/some/sub/directories").unwrap(),
            "github.com/skyezerfox/hooks"
        );
        assert_eq!(
            package_into_git_url("mygitserver.com/skyezerfox/hooks").unwrap(),
            "mygitserver.com/skyezerfox/hooks"
        )
    }

    #[test]
    fn test_resolve_package_url() {
        assert_eq!(
            super::resolve_package_url("skyezerfox/hooks").unwrap(),
            "github.com/skyezerfox/hooks"
        );
        assert_eq!(
            super::resolve_package_url("github.com/skyezerfox/hooks/project").unwrap(),
            "github.com/skyezerfox/hooks/project"
        );
        assert_eq!(
            super::resolve_package_url("mygitserver.com/skyezerfox/hooks/project").unwrap(),
            "mygitserver.com/skyezerfox/hooks/project"
        )
    }

    #[test]
    fn test_extract_package_name() {
        assert_eq!(
            super::extract_package_path("skyezerfox/eslint-hook").unwrap(),
            "."
        );
        assert_eq!(
            super::extract_package_path("github.com/skyezerfox/hooks/project").unwrap(),
            "project"
        );
        assert_eq!(
            super::extract_package_path("mygitserver.com/skyezerfox/hooks/deep/project").unwrap(),
            "deep/project"
        )
    }
}
