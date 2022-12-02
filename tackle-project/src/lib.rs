use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

/// The project manifest definition.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /// The manifest version.
    pub version: String,
	/// Imports of this project.
	pub imports: HashMap<String, Import>,
    /// Pre_commit hooks
	#[serde(default = "Vec::new")]
    pub pre_commit: Vec<HookDefinition>,
	/// Pre_merge_commit hooks
	#[serde(default = "Vec::new")]
    pub pre_merge_commit: Vec<HookDefinition>,
	/// Preapare commit message hooks
	#[serde(default = "Vec::new")]
    pub prepare_commit_msg: Vec<HookDefinition>,
	/// Commit msg hooks
	#[serde(default = "Vec::new")]
    pub commit_msg: Vec<HookDefinition>,
	/// Post_commit hooks
	#[serde(default = "Vec::new")]
	pub post_commit: Vec<HookDefinition>,
	/// Pre-rebase hooks
	#[serde(default = "Vec::new")]
	pub pre_rebase: Vec<HookDefinition>,
}

/// A hook import.
#[derive(Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum Import {
    /// An import specified as a url and version.
    Simple(String),
    /// A verbose import specificed as a table.
    Verbose {
        /// The URL of the package's repository.
        url: String,
        /// The version of the package.
        version: Option<String>,
        /// Hooks included in this project. If `None`, then all hooks (except those specified in `exclude`) are included.
        include: Option<Vec<String>>,
        /// Hooks excluded from this project. If empty, then all hooks specified in `include` are included.
        #[serde(default = "Vec::new")]
        exclude: Vec<String>,
    },
}

/// A hook definition insin array of hook definitions.de the manifest.
#[derive(Deserialize)]
pub struct HookDefinition {
    pub name: Option<String>,
}

/// A data-structure containing project information.
pub struct Project {
    /// The manifest loaded for this project.
    pub manifest: Manifest,
	/// The location of the project.
	pub location: PathBuf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_manifest() {
        let manifest = include_str!("../tests/tackle.example.toml");
        let manifest: Manifest = toml::from_str(manifest).unwrap();
    }
}
  