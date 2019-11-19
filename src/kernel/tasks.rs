use crate::KernelError;

use crate::priv_execute;
use crate::system::task_manager::*;
use crate::utils::arch::svc_call;
use cortex_m::peripheral::syst::SystClkSource;
use crate::utils::arch::pendSV_handler;
use cortex_m::interrupt::{Mutex, free as execute_critical};
use core::cell::RefCell;

use cortex_m::Peripherals;

use crate::system::types::{BooleanVector, TaskId};
use crate::utils::arch::is_privileged;
use cortex_m_semihosting::hprintln;

static empty_task: TaskControlBlock = TaskControlBlock { sp: 0 };

// GLOBALS:
pub static scheduler: Mutex<RefCell<Scheduler>> = Mutex::new(RefCell::new(Scheduler::new()));

pub static os_curr_task_id: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));
pub static os_next_task_id: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));
// end GLOBALS

/// Initialize the switcher system
pub fn init(is_preemptive: bool) {
    execute_critical(|cs_token| unsafe { scheduler.borrow(cs_token).borrow_mut().init(is_preemptive) })
}

// The below section just sets up the timer and starts it.
pub fn start_kernel(peripherals: &mut Peripherals, tick_interval: u32) -> Result<(), KernelError> {
    priv_execute!({
        let syst = &mut peripherals.SYST;
        syst.set_clock_source(SystClkSource::Core);
        syst.set_reload(tick_interval);
        syst.enable_counter();
        syst.enable_interrupt();

        execute_critical(|cs_token| unsafe { scheduler.borrow(cs_token).borrow_mut().start_kernel() });
        preempt();
        Ok(())
    })
}

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
        execute_critical(|cs_token| unsafe {
            scheduler.borrow(cs_token).borrow_mut().create_task(priority as usize, stack, handler_fn, param)
        })
    })
}

pub fn schedule() {
    if is_privileged() == true {
        preempt();
    } else {
        svc_call();
    }
}

pub fn preempt()  {
    execute_critical(|cs_token| {
        let handler = unsafe { &mut scheduler.borrow(cs_token).borrow_mut() };
        let next_tid = handler.get_next_tid() as usize;
        let curr_tid = handler.curr_tid as usize;
        if handler.is_running {
            if curr_tid != next_tid {
                if handler.started {
                    os_curr_task_id.borrow(cs_token).replace(curr_tid);
                } else {
                    handler.started = true;
                }
                handler.curr_tid = next_tid;
                os_next_task_id.borrow(cs_token).replace(next_tid);
            }
        }
    });
    unsafe {
        cortex_m::peripheral::SCB::set_pendsv();
    }
    hprintln!("done");
}

pub fn is_preemptive() -> bool {
    execute_critical(|cs_token| unsafe { scheduler.borrow(cs_token).borrow_mut().is_preemptive })
}

pub fn get_curr_tid() -> TaskId {
    execute_critical(|cs_token| {
        let handler = scheduler.borrow(cs_token).borrow();
        return handler.curr_tid as TaskId;
    })
}

pub fn block_tasks(tasks_mask: BooleanVector) {
    execute_critical(|cs_token| unsafe {
        scheduler.borrow(cs_token).borrow_mut().block_tasks(tasks_mask);
    })
}

pub fn unblock_tasks(tasks_mask: BooleanVector) {
    execute_critical(|cs_token| unsafe {
        scheduler.borrow(cs_token).borrow_mut().unblock_tasks(tasks_mask);
    })
}

pub fn task_exit() {
//    let curr_tid = get_curr_tid();
    execute_critical(|cs_token| {
        hprintln!("wdfsdf");
        let handler = &mut scheduler.borrow(cs_token).borrow_mut();
        unsafe { handler.active_tasks &= !(1 << handler.curr_tid as u32) };
    });
    schedule()
}

pub fn release(tasks_mask: BooleanVector) {
    execute_critical(|cs_token| unsafe { scheduler.borrow(cs_token).borrow_mut().release(tasks_mask) });
}

pub fn enable_preemption() {
    execute_critical(|cs_token| unsafe { scheduler.borrow(cs_token).borrow_mut().is_preemptive = true })
}

pub fn disable_preemption() {
    execute_critical(|cs_token| unsafe {
        scheduler.borrow(cs_token).borrow_mut().is_preemptive = false;
    })
}
