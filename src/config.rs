use serde::{Deserialize, Serialize};

pub static DEFAULT_MANIFEST: &'static str = include_str!("assets/tackle.toml");
pub static DEFAULT_GITIGNORE: &'static str = include_str!("assets/.gitignore");

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TackleManifestHook {
    pub url: String,
    pub version: String,
    pub integrity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TackleManifestHooks {
    #[serde(default = "Vec::new")]
    pub precommit: Vec<TackleManifestHook>,
    #[serde(default = "Vec::new")]
    pub postcommit: Vec<TackleManifestHook>,
    #[serde(default = "Vec::new")]
    pub prepush: Vec<TackleManifestHook>,
    #[serde(default = "Vec::new")]
    pub postpush: Vec<TackleManifestHook>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TackleManifest {
    pub version: String,
    pub hooks: TackleManifestHooks,
}

/// A `tackle.toml` file defining a hook package.
#[derive(Deserialize)]
pub struct Package {
    /// The name of the package.
    pub name: Option<String>,
    /// A description of the package.
    pub description: Option<String>,
    /// The version of the package.
    pub version: Option<String>,
    /// Hooks defined by this package.
    pub hooks: HookDefinitions,
}

/// A collection of hooks defined by a package.
#[derive(Deserialize)]
pub struct HookDefinitions {
    /// A list of hook definitions for the pre-commit hook.
    #[serde(default = "Vec::new")]
    pub precommit: Vec<HookDefinition>,
    /// A list of hook definitions for the post-commit hook.
    #[serde(default = "Vec::new")]
    pub postcommit: Vec<HookDefinition>,
}

/// A hook definition inside a Tackle package.
#[derive(Deserialize, PartialEq, Debug)]
pub struct HookDefinition {
    /// The ID of the hook. This field is used to identify the hook in
    /// condition blocks of other hooks.
    pub id: Option<String>,
    /// The command to run.
    pub command: Vec<String>,
    /// OS-level dependencies for the hook.
    #[serde(default = "Vec::new")]
    pub dependencies: Vec<String>,
    /// A vector of conditions to test before the hook is run.
    #[serde(default = "Vec::new")]
    pub conditions: Vec<HookCondition>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct HookCondition {
    /// Matches successful tasks.
    #[serde(default = "Vec::new")]
    pub successful: Vec<String>,
    /// Matches failed tasks.
    #[serde(default = "Vec::new")]
    pub failed: Vec<String>,
    /// Matches skipped tasks.
    #[serde(default = "Vec::new")]
    pub skipped: Vec<String>,
    /// Matches files.
    #[serde(default = "Vec::new")]
    pub exists: Vec<String>,
    /// Matches the current branch.
    #[serde(default = "Vec::new")]
    pub branch: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::{TackleManifest, DEFAULT_MANIFEST};

    use super::Package;

    #[test]
    fn test_parse() {
        let package: Package = toml::from_str(
            r#"
name = "Example hook"
version = "1.0.0"

[[hooks.precommit]]
id = "example"
command = ["echo Hello, world!"]

[[hooks.precommit.conditions]]
successful = ["example"]
"#,
        )
        .unwrap();

        assert_eq!(package.name, Some("Example hook".to_string()));
        assert_eq!(package.version, Some("1.0.0".to_string()));
        assert_eq!(package.hooks.precommit.len(), 1);
        assert_eq!(package.hooks.precommit[0].id, Some("example".to_string()));
        assert_eq!(
            package.hooks.precommit[0].command[0],
            "echo Hello, world!".to_string()
        );
        assert_eq!(package.hooks.precommit[0].conditions.len(), 1);
        assert_eq!(
            package.hooks.precommit[0].conditions[0].successful[0],
            "example".to_string()
        );
    }

    #[test]
    fn test_default_config_file() {
        let manifest: TackleManifest = toml::from_str(DEFAULT_MANIFEST).unwrap();
        assert_eq!(manifest.version, "1");
        let precommit = manifest.hooks.precommit;
        assert_eq!(precommit.len(), 1);
    }
}
