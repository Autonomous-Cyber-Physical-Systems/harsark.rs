//! # Task Management module
//! Defines Kernel routines which will take care of Task management functionality.
//! Declares a global instance of Scheduler that will be used by the Kernel routines to provide the functionality.

use cortex_m::interrupt::free as execute_critical;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::Peripherals;

use crate::KernelError;
use crate::priv_execute;
use crate::system::task_manager::*;
use crate::utils::arch::svc_call;
use crate::system::types::{BooleanVector, TaskId};
use crate::utils::arch::is_privileged;

static empty_task: TaskControlBlock = TaskControlBlock { sp: 0 };

// GLOBALS:
/// Global Scheduler instance
pub static mut all_tasks: Scheduler = Scheduler::new();
#[no_mangle]
/// Reference to TCB of currently running task
static mut os_curr_task: &TaskControlBlock = &empty_task;
#[no_mangle]
/// Reference to TCB of next to be scheduled task
static mut os_next_task: &TaskControlBlock = &empty_task;
// end GLOBALS

/// Initializes the Kernel scheduler. `is_preemptive` defines if the Kernel should operating preemptively 
/// or not. This method sets the `is_preemptive` field of the Scheduler instance and creates the idle task. 
/// The idle task is created with zero priority; hence, it is only executed when no other task is in Ready state.
pub fn init(is_preemptive: bool) {
    execute_critical(|_| unsafe { all_tasks.init(is_preemptive) })
}

/// Starts the Kernel scheduler, which starts scheduling tasks and starts the SysTick timer using the
/// reference of the Peripherals instance and the `tick_interval`. `tick_interval` specifies the
/// frequency of the timer interrupt. The SysTick exception updates the kernel regarding the time
/// elapsed, which is used to dispatch events and schedule tasks.
pub fn start_kernel(peripherals: &mut Peripherals, tick_interval: u32) -> Result<(), KernelError> {
    priv_execute!({
        let syst = &mut peripherals.SYST;
        syst.set_clock_source(SystClkSource::Core);
        syst.set_reload(tick_interval);
        syst.enable_counter();
        syst.enable_interrupt();

        execute_critical(|_| unsafe { all_tasks.start_kernel() });
        preempt()
    })
}

/// Create a new task with the configuration set as arguments passed.
pub fn create_task<T: Sized>(
    priority: TaskId,
    stack: &mut [u32],
    handler_fn: fn(&T) -> !,
    param: &T,
) -> Result<(), KernelError>
where
    T: Sync,
{
    priv_execute!({
        execute_critical(|_| unsafe {
            all_tasks.create_task(priority as usize, stack, handler_fn, param)
        })
    })
}

/// This function is called from both privileged and unprivileged context.
/// Hence if the function is called from privileged context, then `preempt()` is called.
/// Else, the `svc_call()` is executed, this function creates the SVC exception.
/// And the SVC handler calls schedule again. Thus, the permission level is raised to privileged via the exception.
pub fn schedule() {
    if is_privileged() == true {
        preempt();
    } else {
        svc_call();
    }
}

/// If the scheduler is running and the highest priority task and currently running task aren’t the same, 
/// then the `context_switch()` is called.
pub fn preempt() -> Result<(), KernelError> {
    execute_critical(|_| {
        let handler = unsafe { &mut all_tasks };
        let next_tid = handler.get_next_tid();
        let curr_tid = handler.curr_tid as TaskId;
        if handler.is_running {
            if curr_tid != next_tid {
                context_switch(curr_tid as usize, next_tid as usize);
            }
        }
        return Ok(());
    })
}

/// Assigns the appropriate values to `os_curr_task` and `os_next_task` and raises the PendSV interrupt.
/// PendSV interrupt handler does the actual context switch.
fn context_switch(curr: usize, next: usize) {
    let handler = unsafe { &mut all_tasks };
    let task_curr = &handler.task_control_blocks[curr];
    if handler.started {
        unsafe {
            os_curr_task = task_curr.as_ref().unwrap();
        }
    } else {
        handler.started = true;
    }
    handler.curr_tid = next;
    let task_next = &handler.task_control_blocks[next];
    unsafe {
        os_next_task = task_next.as_ref().unwrap();
        cortex_m::peripheral::SCB::set_pendsv();
    }
}

/// Returns if the scheduler is currently operating preemptively or not.
pub fn is_preemptive() -> bool {
    execute_critical(|_| unsafe { all_tasks.is_preemptive })
}

/// Returns the TaskId of the currently running task in the kernel.
pub fn get_curr_tid() -> TaskId {
    execute_critical(|_| {
        let handler = unsafe { &mut all_tasks };
        return handler.curr_tid as TaskId;
    })
}

/// The Kernel blocks the tasks mentioned in `tasks_mask`.
pub fn block_tasks(tasks_mask: BooleanVector) {
    execute_critical(|_| unsafe {
        all_tasks.block_tasks(tasks_mask);
    })
}

/// The Kernel unblocks the tasks mentioned in tasks_mask.
pub fn unblock_tasks(tasks_mask: BooleanVector) {
    execute_critical(|_| unsafe {
        all_tasks.unblock_tasks(tasks_mask);
    })
}

/// The `task_exit` function is called just after a task finishes execution. This function unsets the task’s corresponding bit in the `active_tasks` and calls schedule. Hence in the next call to schedule, the kernel schedules some other task.
pub fn task_exit() {
    let curr_tid = get_curr_tid();
    execute_critical(|_| {
        unsafe { all_tasks.active_tasks &= !(1 << curr_tid as u32) };
    });
    schedule()
}

/// The Kernel releases the tasks in the `task_mask`, these tasks transition from the waiting to the ready state.
pub fn release(tasks_mask: BooleanVector) {
    execute_critical(|_| unsafe { all_tasks.release(tasks_mask) });
}

/// Enables preemptive scheduling.
pub fn enable_preemption() {
    execute_critical(|_| unsafe { all_tasks.is_preemptive = true })
}

/// Disables preemptive scheduling.
pub fn disable_preemption() {
    execute_critical(|_| unsafe {
        all_tasks.is_preemptive = false;
    })
}
