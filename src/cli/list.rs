use crate::{
    errors::TackleError,
    project::{get_project_root, is_initialized, read_manifest},
};

#[tracing::instrument]
pub fn list() -> Result<(), TackleError> {
    if !is_initialized() {
        return Err(TackleError::NotInitialized);
    }

    let workdir = get_project_root()?;
    let manifest = read_manifest(&workdir)?;

    println!("Pre-commit Hooks:");
    for hook in &manifest.hooks.precommit {
        println!("\t{}", hook.url);
    }
    if manifest.hooks.precommit.is_empty() {
        println!("\tNo hooks installed.");
    }
    println!("Post-commit Hooks:");
    for hook in &manifest.hooks.postcommit {
        println!("\t{}", hook.url);
    }
    if manifest.hooks.precommit.is_empty() {
        println!("\tNo hooks installed.");
    }

    Ok(())
}
