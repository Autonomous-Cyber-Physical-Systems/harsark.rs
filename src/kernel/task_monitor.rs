use core::cell::RefCell;

use crate::kernel::timer::get_time;
use crate::priv_execute;
use crate::system::scheduler::*;
use crate::system::task_monitor::TaskMonitor;
use crate::utils::arch::is_privileged;
use crate::utils::arch::{critical_section, Mutex};
use crate::KernelError;

static TASK_MONITOR: Mutex<RefCell<TaskMonitor>> = Mutex::new(RefCell::new(TaskMonitor::new()));

pub fn set_deadline(tid: TaskId, deadline: u32) {
    critical_section(|cs_token| {
        TASK_MONITOR
            .borrow(cs_token)
            .borrow_mut()
            .set_deadline(tid, get_time() + deadline);
    })
}

pub fn set_handler(handler: fn()) {
    critical_section(|cs_token| {
        TASK_MONITOR
            .borrow(cs_token)
            .borrow_mut()
            .set_handler(handler);
    })
}

pub fn clear_deadline(tid: TaskId) {
    critical_section(|cs_token| {
        TASK_MONITOR
            .borrow(cs_token)
            .borrow_mut()
            .clear_deadline(tid);
    })
}

pub fn sweep_deadlines() {
    critical_section(|cs_token| {
        TASK_MONITOR
            .borrow(cs_token)
            .borrow_mut()
            .sweep_deadlines(get_time());
    })
}
