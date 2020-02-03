//! # Task Management module
//! Defines Kernel routines which will take care of Task management functionality.
//! Declares a global instance of Scheduler that will be used by the Kernel routines to provide the functionality.

use core::cell::RefCell;

use crate::KernelError;
use crate::priv_execute;
use crate::system::scheduler::*;
use crate::utils::arch::{svc_call,Mutex,critical_section};
use crate::utils::helpers::is_privileged;

#[cfg(feature = "logger")]
use crate::kernel::logging; 
#[cfg(feature = "logger")]
use crate::system::logger::LogEventType; 

/// Global Scheduler instance
#[no_mangle]
pub static TaskManager: Mutex<RefCell<Scheduler>> = Mutex::new(RefCell::new(Scheduler::new()));

/// Initializes the Kernel scheduler. `is_preemptive` defines if the Kernel should operating preemptively 
/// or not. This method sets the `is_preemptive` field of the Scheduler instance and creates the idle task. 
/// The idle task is created with zero priority; hence, it is only executed when no other task is in Ready state.
pub fn init() -> Result<(),KernelError>{
    critical_section(|cs_token| TaskManager.borrow(cs_token).borrow_mut().init() )
}

/// Starts the Kernel scheduler, which starts scheduling tasks and starts the SysTick timer using the
/// reference of the Peripherals instance and the `tick_interval`. `tick_interval` specifies the
/// frequency of the timer interrupt. The SysTick exception updates the kernel regarding the time
/// elapsed, which is used to dispatch events and schedule tasks.
pub fn start_kernel() -> ! {
    /**
     * ensure contracts here else panic, for example:  
     * ensure timer has started if events are being used
     * logger initialization is complete 
     * ... etc
     * 
     * address integer overflow throught out the code base and add it to the paper.
     * also write about how size of enum with values is calculated
     * safety in get_msb and all using option enum. go throught return errors.
     */
    loop {
        schedule();
    }
}

/// Create a new task with the configuration set as arguments passed.
pub fn create_task(
    priority: TaskId,
    stack: &mut [u32],
    handler_fn: fn() -> !,
) -> Result<(), KernelError>
{
    priv_execute!({
        critical_section(|cs_token| TaskManager.borrow(cs_token).borrow_mut().create_task(priority as usize, stack, handler_fn))
    })
}
/// This function is called from both privileged and unprivileged context.
/// Hence if the function is called from privileged context, then `preempt()` is called.
/// Else, the `svc_call()` is executed, this function creates the SVC exception.
/// And the SVC handler calls schedule again. Thus, the permission level is raised to privileged via the exception.
pub fn schedule() {
    let is_preemptive = critical_section(|cs_token| TaskManager.borrow(cs_token).borrow_mut().is_preemptive);
    if is_preemptive {
        match is_privileged() {
            true => preempt(),
            false => svc_call(),
        };
    } 
}

fn preempt() {
    unsafe { cortex_m::peripheral::SCB::set_pendsv() }
}

/// Returns the TaskId of the currently running task in the kernel.
pub fn get_curr_tid() -> TaskId {
    critical_section(|cs_token| {
        TaskManager.borrow(cs_token).borrow().curr_tid as TaskId
    })
}

/// The Kernel blocks the tasks mentioned in `tasks_mask`.
pub fn block_tasks(tasks_mask: BooleanVector) {
    if cfg!(feature = "logger") {
        if logging::get_block_tasks_log() {
            logging::report(LogEventType::BlockTasks(tasks_mask));
        }
    }
    critical_section(|cs_token| TaskManager.borrow(cs_token).borrow_mut().block_tasks(tasks_mask))
}

/// The Kernel unblocks the tasks mentioned in tasks_mask.
pub fn unblock_tasks(tasks_mask: BooleanVector) {
    if cfg!(feature = "logger") {
        if logging::get_unblock_tasks_log() {
            logging::report(LogEventType::UnblockTasks(tasks_mask));
        }
    }
    critical_section(|cs_token| TaskManager.borrow(cs_token).borrow_mut().unblock_tasks(tasks_mask))
}

/// The `task_exit` function is called just after a task finishes execution. This function unsets the task’s corresponding bit in the `active_tasks` and calls schedule. Hence in the next call to schedule, the kernel schedules some other task.
pub fn task_exit() {
    critical_section(|cs_token| {
        let handler = &mut TaskManager.borrow(cs_token).borrow_mut();
        let curr_tid = handler.curr_tid;
        if cfg!(feature = "logger") {
            if logging::get_task_exit_log() {
                logging::report(LogEventType::TaskExit(curr_tid as TaskId));
            }
        }
        handler.active_tasks &= !(1 << curr_tid as u32);
    });
    schedule()
}
/// The Kernel releases the tasks in the `task_mask`, these tasks transition from the waiting to the ready state.
pub fn release(tasks_mask: BooleanVector) {
    if cfg!(feature = "logger") {
        if logging::get_release_log() {
            logging::report(LogEventType::ReleaseTasks(tasks_mask));
        }
    }
    critical_section(|cs_token| TaskManager.borrow(cs_token).borrow_mut().release(tasks_mask));
}

pub fn enable_preemption() {
    critical_section(|cs_token| {
        let handler = &mut TaskManager.borrow(cs_token).borrow_mut();
        handler.preempt_disable_count -= 1;
        if handler.preempt_disable_count == 0 {
            handler.is_preemptive = true;
        }
    })
}

pub fn disable_preemption() {
    critical_section(|cs_token| {
        let handler = &mut TaskManager.borrow(cs_token).borrow_mut();
        handler.preempt_disable_count += 1;
        handler.is_preemptive = false;
    })
}
