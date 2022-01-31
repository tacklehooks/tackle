use log::error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TackleError {
    #[error("The current working directory is not a git repository")]
    RepositoryDiscoveryFailed,
    #[error("Failed to write the manifest file")]
    ManifestWriteFailed,
    #[error("Failed to read the manifest file")]
    ManifestReadFailed,
    #[error("Failed to parse the manifest file")]
    ManifestParseFailed(#[from] toml::de::Error),
    #[error("Failed to create the .tackle directory")]
    CreateTackleDirectoryFailed(#[from] std::io::Error),
    #[error("Cannot initialize an existing project")]
    AlreadyInitialized,
    #[error("Cannot perform operation on a non-initialized project")]
    NotInitialized,
    #[error("Invalid commit hook")]
    InvalidCommitHook,
    #[error("Repository clone failed")]
    RepositoryCloneFailed,
}
