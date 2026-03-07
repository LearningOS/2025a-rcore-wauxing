//! Types related to task management

use super::TaskContext;
use super::taskinfo::TaskInfo;

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task info
    pub task_info: TaskInfo,
}

impl TaskControlBlock {
    /// Set the status of the task
    pub fn set_status(&mut self, status: TaskStatus) {
        self.task_status = status;
        self.task_info.status = status;
    }
}
