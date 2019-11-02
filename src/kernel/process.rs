use crate::errors::KernelError;

use crate::interrupts::svc_call;
use crate::priv_execute;
use cortex_m::interrupt::free as execute_critical;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::register::control::Npriv;

use crate::internals::scheduler::*;

use cortex_m::Peripherals;

use crate::internals::helper::check_priv;

static empty_task: TaskControlBlock = TaskControlBlock { sp: 0 };

// GLOBALS:
static mut all_tasks: Scheduler = Scheduler::new();
#[no_mangle]
static mut os_curr_task: &TaskControlBlock = &empty_task;
#[no_mangle]
static mut os_next_task: &TaskControlBlock = &empty_task;
// end GLOBALS

/// Initialize the switcher system
pub fn init(is_preemptive: bool) {
    execute_critical(|_| unsafe { all_tasks.init(is_preemptive) })
}

// The below section just sets up the timer and starts it.
pub fn start_kernel(perif: &mut Peripherals, tick_interval: u32) -> Result<(), KernelError> {
    priv_execute!({
        let syst = &mut perif.SYST;
        syst.set_clock_source(SystClkSource::Core);
        syst.set_reload(tick_interval);
        syst.enable_counter();
        syst.enable_interrupt();

        execute_critical(|_| unsafe { all_tasks.start_kernel() });
        preempt()
    })
}

pub fn create_task<T: Sized>(
    priority: usize,
    stack: &mut [u32],
    handler_fn: fn(&T) -> !,
    param: &T,
) -> Result<(), KernelError> {
    priv_execute!({
        execute_critical(|_| unsafe { all_tasks.create_task(priority, stack, handler_fn, param) })
    })
}

pub fn schedule() {
    let ctrl_reg = cortex_m::register::control::read();
    if ctrl_reg.npriv() == Npriv::Privileged {
        preempt();
    } else {
        svc_call();
    }
}

fn preempt() -> Result<(), KernelError> {
    execute_critical(|_| {
        let handler = unsafe { &mut all_tasks };
        let HT = handler.get_next_tid();
        if handler.is_running {
            if handler.curr_tid != HT {
                context_switch(handler.curr_tid, HT);
            }
        }
        return Ok(());
    })
}

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

pub fn is_preemptive() -> bool {
    execute_critical(|_| unsafe { all_tasks.is_preemptive })
}

pub fn get_pid() -> usize {
    execute_critical(|_| {
        let handler = unsafe { &mut all_tasks };
        return handler.curr_tid;
    })
}

pub fn block_tasks(tasks_mask: u32) {
    execute_critical(|_| unsafe {
        all_tasks.blocked_tasks |= tasks_mask;
    })
}

pub fn unblock_tasks(tasks_mask: u32) {
    execute_critical(|_| unsafe {
        all_tasks.blocked_tasks &= !tasks_mask;
    })
}

pub fn task_exit() {
    execute_critical(|_| {
        let rt = get_pid();
        unsafe { all_tasks.active_tasks &= !(1 << rt as u32) };
    });
    schedule()
}

pub fn release(tasks_mask: u32) -> Result<(), KernelError> {
    execute_critical(|_| {
        unsafe { all_tasks.release(tasks_mask) };
    });
    Ok(())
}

pub fn enable_preemption() {
    execute_critical(|_| {
        unsafe { all_tasks.is_preemptive = true };
    })
}

pub fn disable_preemption() {
    execute_critical(|_| unsafe {
        all_tasks.is_preemptive = false;
    })
}
