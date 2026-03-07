//! Process management syscalls
use crate::{
    mm,
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_syscall_times,
        mmap_current_task, munmap_current_task, suspend_current_and_run_next,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let time_us = get_time_us();
    let time_val = TimeVal {
        sec: time_us / 1_000_000,
        usec: time_us % 1_000_000,
    };
    let token = current_user_token();
    let time_val_slices =
        mm::translated_byte_buffer(token, ts as *const u8, core::mem::size_of::<TimeVal>());
    let time_bytes_slice = unsafe {
        core::slice::from_raw_parts(
            &time_val as *const _ as *const u8,
            core::mem::size_of::<TimeVal>(),
        )
    };
    let mut offset = 0;
    for slice in time_val_slices {
        let len = slice.len();
        slice.copy_from_slice(&time_bytes_slice[offset..offset + len]);
        offset += slice.len();
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    const TRACE_READ: usize = 0;
    const TRACE_WRITE: usize = 1;
    const TRACE_CHECK: usize = 2;

    match trace_request {
        TRACE_READ => {
            let buffers = mm::translated_byte_buffer(current_user_token(), id as *const u8, 1);
            // 安全检查
            if buffers.is_empty() || buffers[0].is_empty() {
                return -1;
            }
            buffers[0][0] as isize
        }
        TRACE_WRITE => {
            let mut buffers = mm::translated_byte_buffer(current_user_token(), id as *mut u8, 1);
            // 安全检查
            if buffers.is_empty() || buffers[0].is_empty() {
                return -1;
            }
            buffers[0][0] = data as u8;
            0
        }
        TRACE_CHECK => get_syscall_times(id),
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if len == 0 {
        return 0;
    }
    let start_va = mm::VirtAddr::from(start);
    let not_aligned = !start_va.aligned();
    let port_not_valid = ((port & !0x7) != 0) || ((port & 0x7) == 0);
    let ptable = mm::PageTable::from_token(current_user_token());
    let start_vpn = start_va.floor();
    let end_vpn = mm::VirtAddr::from(start + len).ceil();
    let mut page_mapped: bool = false;
    for i in start_vpn.0..end_vpn.0 {
        if ptable
            .translate(mm::VirtPageNum::from(i))
            .map_or(false, |pte| pte.is_valid())
        {
            page_mapped = true;
            break;
        }
    }
    if not_aligned || port_not_valid || page_mapped {
        -1
    } else {
        mmap_current_task(start_va, mm::VirtAddr::from(start_va.0 + len), port);
        0
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if len == 0 {
        return 0;
    }
    let start_va = mm::VirtAddr::from(start);
    let not_aligned = !start_va.aligned();
    let ptable = mm::PageTable::from_token(current_user_token());
    let start_vpn = start_va.floor();
    let end_vpn = mm::VirtAddr::from(start + len).ceil();
    let mut page_not_mapped: bool = false;
    for i in start_vpn.0..end_vpn.0 {
        if ptable
            .translate(mm::VirtPageNum::from(i))
            .map_or(true, |pte| !pte.is_valid())
        {
            page_not_mapped = true;
            break;
        }
    }
    if not_aligned || page_not_mapped {
        -1
    } else {
        munmap_current_task(start_va, mm::VirtAddr::from(start_va.0 + len));
        0
    }
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
