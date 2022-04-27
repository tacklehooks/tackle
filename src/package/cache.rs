//! Handles caching of hook packages.

use std::{fmt::Debug, fs, path::PathBuf, sync::Mutex};

use lazy_static::lazy_static;
use log::debug;

use crate::{
    errors::TackleError,
    package::Package,
    util::{extract_package_path, resolve_package_url},
};

lazy_static! {
    /// The path to the project root. This is cached to avoid repeated calls to `get_project_root`.
    pub static ref CACHE_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);
}

/// Resolve the location of the tackle cache directory.
/// If the directory does not exist, it will be created.
#[tracing::instrument]
pub fn resolve_cache_directory() -> Result<PathBuf, TackleError> {
    // check if the cache directory was already resolved
    if let Some(cache_dir) = CACHE_DIR.lock().unwrap().as_ref() {
        debug!("Using cached directory: {}", cache_dir.display());
        return Ok(cache_dir.clone());
    }
    debug!("Resolving the location of the cache directory...");
    let mut cache_dir = dirs::home_dir().unwrap();
    cache_dir.push(".tackle");
    // check if the directory exists, and create it if it doesn't
    if !cache_dir.exists() {
        debug!("Creating the cache directory...");
        fs::create_dir_all(&cache_dir)?;
    }
    Ok(cache_dir)
}

/// Lookup the location of the repository for a particular package.
#[tracing::instrument]
pub fn lookup_repository<S: AsRef<str> + Debug>(url: S) -> Result<Option<PathBuf>, TackleError> {
    let cache_dir = resolve_cache_directory()?;
    let mut path = cache_dir;
    path.push(url.as_ref());
    // resolve the package url
    let path: PathBuf = resolve_package_url(path.to_str().unwrap())?.into();

    if path.exists() {
        debug!("Found package at '{}'", path.display());
        Ok(Some(path))
    } else {
        debug!("Package not found at '{}'", path.display());
        Ok(None)
    }
}

#[tracing::instrument]
pub fn lookup_package<S: AsRef<str> + Debug>(url: S) -> Result<Option<Package>, TackleError> {
    // lookup the repository the package is in
    let package_dir = lookup_repository(&url)?;
    if let None = package_dir {
        return Ok(None);
    }
    let mut package_dir = package_dir.unwrap();
    // get the package name
    let url = extract_package_path(&url)?;
    package_dir.push(url);
    // test if the package directory exists and read its manifest
    if package_dir.exists() && package_dir.is_dir() {
        let package = Package::from_path(&package_dir)?;
        Ok(Some(package))
    } else {
        Ok(None)
    }
}
