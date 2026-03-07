use super::TaskStatus;
use crate::config::MAX_SYSCALL_NUM;

/// The info of a task
#[derive(Copy, Clone)]
pub struct TaskInfo {
    /// Task ID
    pub id: usize,
    /// Task status
    pub status: TaskStatus,
    /// Syscall records array
    pub syscall_records: [usize; MAX_SYSCALL_NUM],
}

impl TaskInfo {
    /// Initialize a new TaskInfo
    pub fn init() -> Self {
        let syscall_records: [usize; MAX_SYSCALL_NUM] = [0; MAX_SYSCALL_NUM];
        Self {
            id: 0,
            status: TaskStatus::UnInit,
            syscall_records,
        }
    }
}
