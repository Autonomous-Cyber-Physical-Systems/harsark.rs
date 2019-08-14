use core::ptr;

//use core::cell::RefCell;
use crate::errors::KernelError;
use core::f64::MAX;
use cortex_m::interrupt::free as execute_critical;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_semihosting::hprintln;

use crate::config::{MAX_TASKS, SYSTICK_INTERRUPT_INTERVAL};

#[repr(C)]
struct TaskState {
    // start fields used in assembly, do not change their order
    ptr_RT: usize, // pointer to running task
    ptr_HT: usize, // pointer to current high priority task (or the next task to be scheduled)
    // end fields used in assembly
    RT: usize,
    is_running: bool,
    threads: [Option<TaskControlBlock>; MAX_TASKS],
    BTV: u32,
    ATV: u32,
}

/// A single thread's state
#[repr(C)]
#[derive(Clone, Copy)]
struct TaskControlBlock {
    // fields used in assembly, do not reorder them
    sp: usize, // current stack pointer of this thread
}

// GLOBALS:
#[no_mangle]
static mut __CORTEXM_THREADS_GLOBAL_PTR: u32 = 0;
static mut __CORTEXM_THREADS_GLOBAL: TaskState = TaskState {
    ptr_RT: 0,
    ptr_HT: 0,
    RT: 0,
    is_running: false,
    threads: [None; MAX_TASKS],
    ATV: 0,
    BTV: 0,
};
pub static mut IS_PREEMPTIVE: bool = false;
// end GLOBALS

/// Initialize the switcher system
pub fn init(is_preemptive: bool) {
    execute_critical(|_| unsafe {
        let ptr: usize = core::intrinsics::transmute(&__CORTEXM_THREADS_GLOBAL);
        __CORTEXM_THREADS_GLOBAL_PTR = ptr as u32;
        __CORTEXM_THREADS_GLOBAL.is_running = true;
        IS_PREEMPTIVE = is_preemptive;
    });
}

// The below section just sets up the timer and starts it.
pub fn start_kernel() -> Result<(), KernelError> {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(SYSTICK_INTERRUPT_INTERVAL);
    syst.enable_counter();
    syst.enable_interrupt();
    unsafe {
        __CORTEXM_THREADS_GLOBAL.is_running = true;
    }
    preempt()?;
    return Ok(());
}

pub fn release(tasks_mask: &u32) {
    execute_critical(|_| {
        let handler = unsafe { &mut __CORTEXM_THREADS_GLOBAL };
        handler.ATV |= *tasks_mask;
    });
}

pub fn create_task(
    priority: usize,
    stack: &mut [u32],
    handler_fn: fn() -> !,
) -> Result<(), KernelError> {
    match create_tcb(stack, handler_fn, true) {
        Ok(tcb) => {
            insert_tcb(priority, tcb)?;
            return Ok(());
        }
        Err(e) => return Err(e),
    }
}

pub fn preempt() -> Result<(), KernelError> {
    execute_critical(|_| {
        let handler = unsafe { &mut __CORTEXM_THREADS_GLOBAL };
        if handler.is_running {
            let HT = get_HT();
            // schedule a thread to be run
            if handler.RT != HT {
                handler.RT = HT;
                let task = &handler.threads[handler.RT];
                if let Some(task) = task {
                    unsafe {
                        handler.ptr_HT = core::intrinsics::transmute(task);
                        // The following Causes PendSV interrupt, the interrupt handler is written in assembly
                        let pend = ptr::read_volatile(0xE000ED04 as *const u32);
                        ptr::write_volatile(0xE000ED04 as *mut u32, pend | 1 << 28);
                    }
                } else {
                    return Err(KernelError::DoesNotExist);
                }
            }
        }
        return Ok(());
    })
}

fn get_HT() -> usize {
    execute_critical(|_| {
        let handler = unsafe { &mut __CORTEXM_THREADS_GLOBAL };
        for i in (1..MAX_TASKS as u32).rev() {
            let i_mask = (1 << i);
            if (handler.ATV & i_mask == i_mask) && (handler.BTV & i_mask != i_mask) {
                return i as usize;
            }
        }
        return 0;
    })
}

fn create_tcb(
    stack: &mut [u32],
    handler: fn() -> !,
    priviliged: bool,
) -> Result<TaskControlBlock, KernelError> {
    if stack.len() < 32 {
        return Err(KernelError::StackTooSmall);
    }

    let idx = stack.len() - 1;
    stack[idx] = 1 << 24; // xPSR
    let pc: usize = handler as usize;
    stack[idx - 1] = pc as u32; // PC
    stack[idx - 2] = 0xFFFFFFFD; // LR
    stack[idx - 3] = 0xCCCCCCCC; // R12
    stack[idx - 4] = 0x33333333; // R3
    stack[idx - 5] = 0x22222222; // R2
    stack[idx - 6] = 0x11111111; // R1
    stack[idx - 7] = 0x00000000; // R0
                                 // aditional regs
    stack[idx - 08] = 0x77777777; // R7
    stack[idx - 09] = 0x66666666; // R6
    stack[idx - 10] = 0x55555555; // R5
    stack[idx - 11] = 0x44444444; // R4
    stack[idx - 12] = 0xBBBBBBBB; // R11
    stack[idx - 13] = 0xAAAAAAAA; // R10
    stack[idx - 14] = 0x99999999; // R9
    stack[idx - 15] = 0x88888888; // R8

    let sp: usize = unsafe { core::intrinsics::transmute(&stack[stack.len() - 16]) };
    let tcb = TaskControlBlock { sp: sp as usize };
    Ok(tcb)
}

fn insert_tcb(idx: usize, tcb: TaskControlBlock) -> Result<(), KernelError> {
    execute_critical(|_| {
        let handler = unsafe { &mut __CORTEXM_THREADS_GLOBAL };
        if idx >= MAX_TASKS {
            return Err(KernelError::DoesNotExist);
        }
        handler.threads[idx] = Some(tcb);
        return Ok(());
    })
}

pub fn get_RT() -> usize {
    execute_critical(|_| {
        let handler = unsafe { &mut __CORTEXM_THREADS_GLOBAL };
        return handler.RT;
    })
}
