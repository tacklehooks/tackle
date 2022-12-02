use std::collections::HashMap;

/// An enumeration of task conditions.
enum Condition {
	/// The task always runs, regardless of the previous task's fail state.
	Always,
	/// The task runs if the specified tasks were successful.
	RequireSuccess(Vec<TaskId>),
	/// The task runs if the specified tasks were not successful
	RequireFailure(Vec<TaskId>),
	/// The task is disabled and never runs.
	Disabled
}

/// A data-structure containing the ID of a task.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TaskId {
	id: usize,
}

/// A data-structure containing task information.
struct Task {
	/// The condition required for this task to run.
	pub condition: Condition,
	/// The type of the task.
	pub ty: TaskType
}

/// An enumeration of task types.
enum TaskType {
	/// Execute a given shell command.
	Exec(String)
}

/// Stores information about the task graph.
struct TaskGraph {
	/// A hash map of every task in the graph.
	tasks: HashMap<TaskId, Task>,
	/// The first task in the graph
	head: Option<TaskId>
}
