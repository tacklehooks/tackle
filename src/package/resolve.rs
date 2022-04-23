//! This module contains the code for resolving and fetching packages.

use std::{future::Future, path::PathBuf};

use futures::future::join_all;
use semver::Version;
use tempdir::TempDir;
use tokio::{join, task::JoinHandle};
use url::Url;

use crate::errors::TackleError;

/// This struct represents a package resolved by the resolution step.
pub struct ResolvedPackage {
    /// The url or path to the package.
    pub url: ResolvedPackageUrl,
    /// The name of this package.
    pub name: String,
    /// The version of this package.
    pub version: Version,
}

/// Enum containing the two possible package locations.
pub enum ResolvedPackageUrl {
    Remote(Url),
    Cache(PathBuf),
}

/// Resolves packages from its internal repository, and the package cahce.
pub struct PackageResolver {
    pub repositories: Vec<Repository>,
    // TODO: pub cache: Cache,
}

impl PackageResolver {
    /// Resolves a package from the repository.
    pub async fn resolve(&self, name: String, version: Version) -> Option<ResolvedPackage> {
        // spawn resolution tasks
        let handles: Vec<JoinHandle<_>> = self
            .repositories
            .iter()
            .map(|repo| {
                let name = name.clone();
                let version = version.clone();
                let repo = (*repo).clone();
                tokio::spawn(async move { repo.resolve(name, version).await })
            })
            .collect();
        // find the first successful resolution
        let results: Vec<_> = join_all(handles)
            .await
            .into_iter()
            // join errors should never occur
            .map(|r| r.unwrap())
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .collect();
        // consume the results vec and return the first result
        results.into_iter().next()
    }
}

/// An immutable representation of a repository of Tackle packages.
#[derive(Debug, Clone)]
pub struct Repository {
    pub url: Url,
}

impl Repository {
    /// Resolves a package from the repository.
    pub async fn resolve(
        &self,
        name: String,
        version: Version,
    ) -> Result<Option<ResolvedPackage>, TackleError> {
        // resolve package url
        let package_url = self
            .url
            .join(&name)
            .map_err(|e| TackleError::InvalidCommitHook)?;
        // attempt to clone repository
        let repository = git2::Repository::clone(
            &package_url.to_string(),
            TempDir::new(&format!("{}-{}", name, version))
                .map_err(|_| TackleError::AlreadyInitialized)?,
        )
        .map_err(|e| TackleError::RepositoryCloneFailed)?;

        let tags = repository.tag_names(None).unwrap();
        // find matching tag
        let tag = tags
            .into_iter()
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .find(|s| s == &version.to_string());
        // if no matching tag, return None
        if tag.is_none() {
            return Ok(None);
        }
        todo!()
    }
}
