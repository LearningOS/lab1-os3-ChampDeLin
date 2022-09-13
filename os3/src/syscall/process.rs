//! Process management syscalls

use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}

static mut num: [u32; MAX_SYSCALL_NUM] = [0; MAX_SYSCALL_NUM];

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    unsafe {
        num[92] = num[92] + 1;
    }
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    unsafe {
        num[123] = num[123] + 1;
    }
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        num[168] = num[168] + 1;
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

static mut old_time: usize = 0;
/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    unsafe {
        num[409] = num[409] + 1;
        let cur_time:usize = get_time_us();
        let cha = cur_time - old_time;
        old_time = cur_time;

        *ti = TaskInfo {
            status: TaskStatus::Running,
            syscall_times: num,
            time: cha,
        };
    }
    -1
}
