use crate::config::MAX_SYSCALL_NUM;
use super::TaskStatus;	

/// The info of a task
#[derive(Copy, Clone)]
pub struct TaskInfo {
    pub id: usize,
    pub status: TaskStatus,
    pub syscall_records: [usize ; MAX_SYSCALL_NUM],
}

impl TaskInfo {
    pub fn init() -> Self {
        let mut syscall_records: [usize; MAX_SYSCALL_NUM] = Default::default();
        Self {
            id: 0,
            status: TaskStatus::UnInit,
            syscall_records,
        }
    }
}
