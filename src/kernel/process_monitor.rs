use core::cell::RefCell;

use crate::KernelError;
use crate::priv_execute;
use crate::system::scheduler::*;
use crate::utils::arch::{svc_call,Mutex,critical_section};
use crate::utils::helpers::is_privileged;
use crate::system::process_monitor::ProcessMonitor;

static ProcMonitor: Mutex<RefCell<ProcessMonitor>> = Mutex::new(RefCell::new(ProcessMonitor::new()));

pub fn set_deadline(tid: TaskId, deadline: u32) {
    critical_section(|cs_token| {
        ProcMonitor.borrow(cs_token).borrow_mut().set_deadline(tid, deadline);
    })
}

pub fn clear_deadline(tid: TaskId) {
    critical_section(|cs_token| {
        ProcMonitor.borrow(cs_token).borrow_mut().clear_deadline(tid);
    })
}

pub fn timer_update() {
    critical_section(|cs_token| {
        ProcMonitor.borrow(cs_token).borrow_mut().update_timer();
    })
}