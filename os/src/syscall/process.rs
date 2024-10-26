//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, get_current_task_status, suspend_current_and_run_next, TaskStatus},
    timer::{get_time_ms, get_time_us},
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
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    // syscall ID: 410
    // 查询当前正在执行的任务信息，任务信息包括任务控制块相关信息（任务状态）、任务使用的系统调用及调用次数、系统调用时刻距离任务第一次被调度时刻的时长（单位ms）。
    let mut current_task_status = get_current_task_status();
    current_task_status.task_syscall_times[410] += 1;
    unsafe {
        (*_ti) = TaskInfo {
            status: current_task_status.task_status,
            syscall_times: current_task_status.task_syscall_times,
            time: get_time_ms() - current_task_status.task_start_time_ms,
        }

    }

    // 首先找到当前正在执行的任务
    // 怎么找到任务

    // 找到任务使用的系统调用及调用次数

    // 找到系统调用时刻距离任务第一次被调度时刻的时长（单位ms）
    0
}
