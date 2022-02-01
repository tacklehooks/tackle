//! Handles hook conditions.
use crate::{config::HookDefinition, os::is_program_in_path};

/// An enum of possible hook states.
#[derive(PartialEq)]
pub enum HookState {
    /// The hook was successful.
    Successful,
    /// The hook exited with a non-zero error code.
    Failed,
    /// The hook was skipped as its conditions were not met.
    Skipped,
    /// The hook is waiting to be run.
    Pending,
}

/// Wrapper struct for a hook definition, containing the hook definition and
/// the state of the hook.
#[derive(PartialEq)]
pub struct HookWithState {
    hook: HookDefinition,
    state: HookState,
}

/// The hook runner runs hooks!
pub struct HookRunner {
    hooks: Vec<HookWithState>,
}

impl HookRunner {
    /// Creates a new hook runner.
    pub fn from_hooks(hooks: Vec<HookDefinition>) -> HookRunner {
        HookRunner {
            hooks: hooks
                .into_iter()
                .map(|hook| HookWithState {
                    hook,
                    state: HookState::Pending,
                })
                .collect(),
        }
    }

    /// Set the state of a hook.
    pub fn set_hook_state(&mut self, hook_id: &str, state: HookState) {
        let hook = self
            .hooks
            .iter_mut()
            .find(|hook| hook.hook.id == Some(hook_id.to_owned()))
            .expect("Hook not found");

        hook.state = state;
    }

    /// Test if a hook is successful.
    pub fn is_matching_hook(&self, hook: &HookWithState) -> bool {
        // ensure that the hook is pending
        if hook.state != HookState::Pending {
            return false;
        }

        // ensure that hook has os dependencies
        if !hook.hook.dependencies.is_empty() {
            if !hook
                .hook
                .dependencies
                .iter()
                .all(|dep| is_program_in_path(dep))
            {
                return false;
            }
        }

        let repository = git2::Repository::discover(std::env::current_dir().unwrap()).unwrap();

        // check all conditions
        let conditions = &hook.hook.conditions;
        conditions.iter().any(|condition| {
            let matches_skip = condition.skipped.iter().all(|hook_id| {
                self.hooks.iter().any(|hook| {
                    hook.hook.id == Some(hook_id.clone()) && hook.state == HookState::Skipped
                })
            });
            let matches_success = condition.successful.iter().all(|hook_id| {
                self.hooks.iter().any(|hook| {
                    hook.hook.id == Some(hook_id.clone()) && hook.state == HookState::Successful
                })
            });
            let matches_failed = condition.failed.iter().all(|hook_id| {
                self.hooks.iter().any(|hook| {
                    hook.hook.id == Some(hook_id.clone()) && hook.state == HookState::Failed
                })
            });
            let matches_exists = condition
                .exists
                .iter()
                .all(|file| std::fs::metadata(file).is_ok());
            let matches_branch = condition
                .branch
                .iter()
                .all(|branch| repository.head().unwrap().shorthand().unwrap() == branch);

            matches_skip && matches_success && matches_failed && matches_exists && matches_branch
        })
    }

    /// Get the next hook to run, respecting the hook order and conditions.
    pub fn next_hook(&mut self) -> Option<&HookDefinition> {
        self.hooks
            .iter()
            .filter(|hook| self.is_matching_hook(hook))
            .next()
            .map(|hook| &hook.hook)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config::{HookCondition, HookDefinition},
        conditions::HookState,
    };

    use super::HookRunner;

    #[test]
    fn test_hook_conditions() {
        let mut hook_runner = HookRunner::from_hooks(vec![
            HookDefinition {
                id: Some("example-1".to_string()),
                command: vec!["echo".to_string(), "Hello, world!".to_string()],
                dependencies: vec![],
                conditions: vec![HookCondition {
                    successful: vec![],
                    failed: vec![],
                    skipped: vec![],
                    branch: vec![],
                    exists: vec![],
                }],
            },
            HookDefinition {
                id: Some("example-2".to_string()),
                command: vec!["echo".to_string(), "Hello, world!".to_string()],
                dependencies: vec![],
                conditions: vec![HookCondition {
                    successful: vec!["example-1".to_string()],
                    failed: vec![],
                    skipped: vec![],
                    branch: vec![],
                    exists: vec![],
                }],
            },
            HookDefinition {
                id: Some("example-3".to_string()),
                command: vec!["echo".to_string(), "Hello, world!".to_string()],
                dependencies: vec![],
                conditions: vec![HookCondition {
                    successful: vec!["example-1".to_string()],
                    failed: vec!["example-2".to_string()],
                    skipped: vec![],
                    branch: vec![],
                    exists: vec![],
                }],
            },
        ]);

        assert_eq!(
            hook_runner.next_hook().unwrap().id,
            Some("example-1".to_string())
        );
        hook_runner.set_hook_state("example-1", HookState::Successful);
        assert_eq!(
            hook_runner.next_hook().unwrap().id,
            Some("example-2".to_string())
        );
        hook_runner.set_hook_state("example-2", HookState::Successful);
        assert_eq!(hook_runner.next_hook(), None);
    }
}
