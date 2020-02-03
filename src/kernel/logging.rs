use core::cell::RefCell;

use crate::KernelError;
use crate::priv_execute;
use crate::system::scheduler::*;
use crate::utils::arch::{svc_call,Mutex,critical_section};
use crate::utils::helpers::is_privileged;
use crate::system::logger::*;

static Logger: Mutex<RefCell<Logger>> = Mutex::new(RefCell::new(Logger::new()));

pub fn report(event_type: LogEventType) {
    critical_section(|cs_token| {
        // use actual timer.
        Logger.borrow(cs_token).borrow_mut().push(LogEvent::new(event_type, 0));
    })
}

pub fn process<F> (handler: F) 
where
    F: Fn(LogEvent),
{
    critical_section(|cs_token| {
        while let Some(event) = Logger.borrow(cs_token).borrow_mut().pop() {
            handler(event);
        }
    })
}

pub fn set_all(val: bool) {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().release_log = val;
        Logger.borrow(cs_token).borrow_mut().block_tasks_log = val;
        Logger.borrow(cs_token).borrow_mut().unblock_tasks_log = val;
        Logger.borrow(cs_token).borrow_mut().task_exit_log = val;
        Logger.borrow(cs_token).borrow_mut().resource_lock_log = val;
        Logger.borrow(cs_token).borrow_mut().resource_unlock_log = val;
        Logger.borrow(cs_token).borrow_mut().message_broadcast_log = val;
        Logger.borrow(cs_token).borrow_mut().message_recieve_log = val;
        Logger.borrow(cs_token).borrow_mut().semaphore_signal_log = val;
        Logger.borrow(cs_token).borrow_mut().semaphore_reset_log = val;
        Logger.borrow(cs_token).borrow_mut().timer_event_log = val;
    })
}

pub fn set_release_log(val: bool) {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().release_log = val;
    })
}

pub fn set_block_tasks_log(val: bool) {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().block_tasks_log = val;
    })
}

pub fn set_unblock_tasks_log(val: bool) {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().unblock_tasks_log = val;
    })
}

pub fn set_task_exit_log(val: bool) {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().task_exit_log = val;
    })
}

pub fn set_resource_lock_log(val: bool) {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().resource_lock_log = val;
    })
}

pub fn set_resource_unlock_log(val: bool) {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().resource_unlock_log = val;
    })
}

pub fn set_message_broadcast_log(val: bool) {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().message_broadcast_log = val;
    })
}

pub fn set_message_recieve_log(val: bool) {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().message_recieve_log = val;
    })
}

pub fn set_semaphore_signal_log(val: bool) {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().semaphore_signal_log = val;
    })
}

pub fn set_semaphore_reset_log(val: bool) {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().semaphore_reset_log = val;
    })
}

pub fn set_timer_event_log(val: bool) {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().timer_event_log = val;
    })
}

pub fn get_release_log() -> bool {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().release_log
    })
}

pub fn get_block_tasks_log() -> bool {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().block_tasks_log
    })
}

pub fn get_unblock_tasks_log() -> bool {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().unblock_tasks_log
    })
}

pub fn get_task_exit_log() -> bool {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().task_exit_log
    })
}

pub fn get_resource_lock_log() -> bool {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().resource_lock_log
    })
}

pub fn get_resource_unlock_log() -> bool {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().resource_unlock_log
    })
}

pub fn get_message_broadcast_log() -> bool {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().message_broadcast_log
    })
}

pub fn get_message_recieve_log() -> bool {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().message_recieve_log
    })
}

pub fn get_semaphore_signal_log() -> bool {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().semaphore_signal_log
    })
}

pub fn get_semaphore_reset_log() -> bool {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().semaphore_reset_log
    })
}

pub fn get_timer_event_log() -> bool {
    critical_section(|cs_token| {
        Logger.borrow(cs_token).borrow_mut().timer_event_log
    })
}