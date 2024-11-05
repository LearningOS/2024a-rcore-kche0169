//! Process management syscalls
use core::mem;
// use alloc::vec;
use alloc::vec::Vec;
use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE, MEMORY_END}, 
    mm::{modify_struct_field, translated_struct_ptr}, 
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_current_task_start_time, get_syscall_times, mmap, suspend_current_and_run_next, TaskStatus, munmap,
    }, timer::{get_time_ms, get_time_us}
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
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
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let token = current_user_token();
    let time_val_size = mem::size_of::<TimeVal>();
    let time_val_ptr = _ts as *const u8;
    let sec = usize_to_u8_array(get_time_us() / 1_000_000);
    let usec = usize_to_u8_array(get_time_us() % 1_000_000);
    let mut combined = Vec::new();
    combined.extend(&sec);
    combined.extend(&usec);
    modify_struct_field(token, time_val_ptr, time_val_size, combined);
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");

    let ti = translated_struct_ptr(current_user_token(), _ti);

    *ti = TaskInfo {
        status: TaskStatus::Running,
        syscall_times: get_syscall_times(),
        time: get_time_ms() - get_current_task_start_time(),
    };
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap");

    if _start %  PAGE_SIZE != 0 || _port & !0x7 != 0 || _port & 0x7 == 0 || _start + _len > MEMORY_END{
        return -1;
    }
    mmap(_start, _len, _port);
    0

}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap");
    if _start % PAGE_SIZE != 0 {
        return -1;
    }
    munmap(_start, _len);
    0
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
/// 拆分
// fn u8_array_to_usize(bytes: &[u8]) -> usize {
//     let mut value: usize = 0;
//     for (i, &byte) in bytes.iter().enumerate() {
//         value |= (byte as usize) << (i * 8); // 将每个字节左移并合并
//     }
//     value
// }
/// dox
fn usize_to_u8_array(value: usize) -> [u8; mem::size_of::<usize>()] {
    let mut bytes = [0u8; mem::size_of::<usize>()];
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = (value >> (i * 8)) as u8; // 获取每个字节
    }
    bytes
}