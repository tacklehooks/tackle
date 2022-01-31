use git2::Repository;

use crate::{errors::TackleError, get_project_root};

/// Attempt to clone the repository at the given URL into the hook directory.
pub fn fetch_package<S: AsRef<str>>(package: S) -> Result<(), TackleError> {
    let package = package.as_ref();
    let path = get_project_root()?.join(".tackle/hooks").join(package);
    // if the work directory exists, PANIC!!
    if path.exists() {
        panic!("attempting to clone into existing directory");
    }
    // clone the repository
    let repository = Repository::clone(&format!("https://{}.git", package), path)
        .map_err(|err| TackleError::RepositoryCloneFailed)?;
    Ok(())
}
