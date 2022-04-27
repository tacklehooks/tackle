use log::info;

use crate::{
    errors::TackleError,
    package::fetch_package,
    project::{get_project_root, is_initialized, read_manifest, TackleManifestHook},
};

/// Install a git hook from the target repository.
#[tracing::instrument]
pub fn install(url: String) -> Result<(), TackleError> {
    if !is_initialized() {
        return Err(TackleError::NotInitialized);
    }
    let workdir = get_project_root()?;
    let _manifest = read_manifest(&workdir)?;
    // install package
    info!("Installing '{}'...", url);
    let _package = fetch_package(&url)?;
    // create the manifest hook entry
    let _manifest_hook = TackleManifestHook {
        url: url.clone(),
        integrity: "".to_owned(),
        version: "1".to_owned(),
    };
    // // create the new manifest file
    // let append_hook = |mut hooks: Vec<TackleManifestHook>| {
    //     hooks.push(manifest_hook.clone());
    //     hooks
    // };
    // let new_manifest = match hook {
    //     Hook::PreCommit => TackleManifest {
    //         hooks: TackleManifestHooks {
    //             precommit: Some(
    //                 manifest
    //                     .hooks
    //                     .precommit
    //                     .map_or(vec![manifest_hook.clone()], append_hook),
    //             ),
    //             ..manifest.hooks
    //         },
    //         ..manifest
    //     },
    // };
    // // write the new manifest
    // write_manifest(&workdir, &new_manifest)?;

    Ok(())
}
