//! # Task Management module
//! Defines Kernel routines which will take care of Task management functionality.
//! Declares a global instance of Scheduler that will be used by the Kernel routines to provide the functionality.

use core::cell::RefCell;

use crate::priv_execute;
use crate::system::scheduler::*;
use crate::utils::arch::is_privileged;
use crate::utils::arch::{critical_section, set_pendsv, Mutex};
use crate::KernelError;

#[cfg(feature = "system_logger")]
use crate::kernel::logging;
#[cfg(feature = "system_logger")]
use crate::system::system_logger::LogEventType;

/// Global Scheduler instance
#[no_mangle]
pub(crate) static TaskManager: Mutex<RefCell<Scheduler>> =
    Mutex::new(RefCell::new(Scheduler::new()));

/// Initializes the Kernel scheduler and creates the idle task, a task that puts the CPU to sleep in a loop.
/// The idle task is created with zero priority; hence, it is only executed when no other task is in Ready state.
pub fn init(init_handler: fn(cxt: &Context) -> Result<(), KernelError>) -> Result<(), KernelError> {
    critical_section(|cs_token| TaskManager.borrow(cs_token).borrow_mut().init(init_handler))
}

/// Starts the Kernel scheduler, which starts scheduling tasks on the CPU.
pub fn start_kernel() -> ! {
    loop {
        schedule(critical_section(|cs_token| {
            TaskManager.borrow(cs_token).borrow_mut().is_preemptive
        }));
    }
}

#[cfg(feature = "task_monitor")]
/// Create a new task with the configuration set as arguments passed.
pub fn create_task(
    priority: TaskId,
    deadline: u32,
    stack: &mut [u32],
    handler_fn: fn(int) -> !,
) -> Result<(), KernelError> {
    priv_execute!({
        critical_section(|cs_token| {
            TaskManager.borrow(cs_token).borrow_mut().create_task(
                priority as usize,
                deadline,
                stack,
                handler_fn,
            )
        })
    })
}

#[cfg(not(feature = "task_monitor"))]
/// Create a new task with the configuration set as arguments passed.
pub fn create_task(
    priority: TaskId,
    stack: &mut [u32],
    handler_fn: fn(ContextType) -> !,
) -> Result<(), KernelError> {
    priv_execute!({
        critical_section(|cs_token| {
            TaskManager.borrow(cs_token).borrow_mut().create_task(
                priority as usize,
                stack,
                handler_fn,
            )
        })
    })
}
/// This function is called from both privileged and unprivileged context.
/// Hence if the function is called from privileged context, then `preempt()` is called.
/// Else, the `svc_call()` is executed, this function creates the SVC exception.
/// And the SVC handler calls schedule again. Thus, the permission level is raised to privileged via the exception.
pub fn schedule(is_preemptive: bool) {
    if is_preemptive {
        preempt();
    }
}

pub fn preempt() {
    set_pendsv();
}

/// Returns the TaskId of the currently running task in the kernel.
pub fn get_curr_tid() -> TaskId {
    critical_section(|cs_token| TaskManager.borrow(cs_token).borrow().curr_tid as TaskId)
}

/// The Kernel blocks the tasks mentioned in `tasks_mask`.
pub fn block_tasks(tasks_mask: BooleanVector) {
    #[cfg(feature = "system_logger")]
    {
        if logging::get_block_tasks() {
            logging::report(LogEventType::BlockTasks(tasks_mask));
        }
    }
    critical_section(|cs_token| {
        TaskManager
            .borrow(cs_token)
            .borrow_mut()
            .block_tasks(tasks_mask)
    })
}

/// The Kernel unblocks the tasks mentioned in tasks_mask.
pub fn unblock_tasks(tasks_mask: BooleanVector) {
    #[cfg(feature = "system_logger")]
    {
        if logging::get_unblock_tasks() {
            logging::report(LogEventType::UnblockTasks(tasks_mask));
        }
    }
    critical_section(|cs_token| {
        TaskManager
            .borrow(cs_token)
            .borrow_mut()
            .unblock_tasks(tasks_mask)
    })
}

/// The `task_exit` function is called just after a task finishes execution. It marks the current running task as finished and then schedules the next high priority task.
pub fn task_exit() {
    let is_preemptive = critical_section(|cs_token| {
        let handler = &mut TaskManager.borrow(cs_token).borrow_mut();
        let curr_tid = handler.curr_tid;
        #[cfg(feature = "system_logger")]
        {
            if logging::get_task_exit() {
                logging::report(LogEventType::TaskExit(curr_tid as TaskId));
            }
        }
        handler.active_tasks &= !(1 << curr_tid as u32);
        handler.is_preemptive
    });
    schedule(is_preemptive);
}
/// The Kernel releases the tasks in the `task_mask`, these tasks transition from the waiting to the ready state.
pub fn release(tasks_mask: BooleanVector) {
    #[cfg(feature = "system_logger")]
    {
        if logging::get_release() {
            logging::report(LogEventType::ReleaseTasks(tasks_mask));
        }
    }
    critical_section(|cs_token| {
        TaskManager
            .borrow(cs_token)
            .borrow_mut()
            .release(tasks_mask)
    });
}

/// Enable preemptive scheduling
pub fn enable_preemption() {
    critical_section(|cs_token| {
        let handler = &mut TaskManager.borrow(cs_token).borrow_mut();
        handler.preempt_disable_count -= 1;
        if handler.preempt_disable_count == 0 {
            handler.is_preemptive = true;
        }
    })
}

/// Disable preemptive scheduling
pub fn disable_preemption() {
    critical_section(|cs_token| {
        let handler = &mut TaskManager.borrow(cs_token).borrow_mut();
        handler.preempt_disable_count += 1;
        handler.is_preemptive = false;
    })
}
