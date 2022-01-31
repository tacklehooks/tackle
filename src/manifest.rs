use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::errors::TackleError;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TackleManifestHook {
    pub url: String,
    pub version: String,
    pub integrity: String,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct TackleManifestHooks {
    pub precommit: Option<Vec<TackleManifestHook>>,
    pub postcommit: Option<Vec<TackleManifestHook>>,
    pub prepush: Option<Vec<TackleManifestHook>>,
    pub postpush: Option<Vec<TackleManifestHook>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TackleManifest {
    pub version: String,
    pub hooks: TackleManifestHooks,
}

static DEFAULT_MANIFEST: &'static str = include_str!("assets/tackle.toml");
static DEFAULT_GITIGNORE: &'static str = include_str!("assets/.gitignore");

/// Test if the tackle directory exists.
pub fn tackle_directory_exists<P: AsRef<Path>>(workdir: P) -> bool {
    let path = workdir.as_ref().join(".tackle");
    path.exists() && path.is_dir()
}

/// Read the manifest file.
pub fn read_manifest<P: AsRef<Path>>(workdir: P) -> Result<TackleManifest, TackleError> {
    let path = workdir.as_ref().join(".tackle/tackle.toml");
    let contents = fs::read_to_string(&path)?;
    let manifest: TackleManifest =
        toml::from_str(&contents).map_err(|err| TackleError::ManifestParseFailed(err))?;
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
        fs::create_dir_all(&path).map_err(|err| TackleError::CreateTackleDirectoryFailed(err))?;
    }
    // create the empty hooks directory
    let hooks_dir = &path.join("hooks");
    if !hooks_dir.exists() {
        fs::create_dir_all(&hooks_dir)
            .map_err(|err| TackleError::CreateTackleDirectoryFailed(err))?;
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

#[cfg(test)]
mod tests {
    use super::{TackleManifest, DEFAULT_MANIFEST};

    #[test]
    fn test_default_config_file() {
        let manifest: TackleManifest = toml::from_str(DEFAULT_MANIFEST).unwrap();
        assert_eq!(manifest.version, "1");
        let precommit = manifest.hooks.precommit.unwrap();
        assert_eq!(precommit.len(), 1);
    }
}
