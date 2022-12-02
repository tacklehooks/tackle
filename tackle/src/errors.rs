//! Defines the application error type and error handling methods.
use log::error;
use thiserror::Error;

/// Enum representing the possible errors that can occur when interacting with the git hook manager.
#[derive(Error, Debug)]
pub enum TackleError {
    /// An error occurred when interacting with the git repository.
    #[error("The current working directory is not a git repository")]
    RepositoryDiscoveryFailed,
    /// An error occured while writing the manifest file.
    #[error("Failed to write the manifest file")]
    ManifestWriteFailed,
    /// An error occured while reading the manifest file.
    #[error("Failed to read the manifest file")]
    ManifestReadFailed,
    /// An error occured while parsing the manifest file.
    #[error("Failed to parse the manifest file")]
    ManifestParseFailed(#[from] toml::de::Error),
    /// An error occured while creating the tackle directory.
    #[error("Failed to create the .tackle directory")]
    CreateTackleDirectoryFailed(#[from] std::io::Error),
    /// An error occured while creating the hooks directory.
    #[error("Cannot initialize an existing project")]
    AlreadyInitialized,
    /// Attempted to perform an operation on a non-existent project.
    #[error("Cannot perform operation on a non-initialized project")]
    NotInitialized,
    /// Attempted to use an invalid commit hook.
    #[error("Invalid commit hook")]
    InvalidCommitHook,
    /// An error occured while cloning the repository.
    #[error("Repository clone failed")]
    RepositoryCloneFailed,
}
