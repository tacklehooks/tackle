use std::{fs, path::Path};

use log::debug;
use serde::{Deserialize, Serialize};

use crate::{
    config::{TackleManifest, DEFAULT_GITIGNORE, DEFAULT_MANIFEST},
    errors::TackleError,
};

/// Test if the tackle directory exists.
pub fn tackle_directory_exists<P: AsRef<Path>>(workdir: P) -> bool {
    let path = workdir.as_ref().join(".tackle");
    path.exists() && path.is_dir()
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
