#![no_std]

use core::ptr;
use cortex_m::interrupt::{
    disable as disable_interrupt,
    enable as enable_interrupt,
};

use cortex_m::peripheral::{
    syst::SystClkSource,
    SYST
};

use cortex_m_semihosting::hprintln;

pub static ERR_TOO_MANY_THREADS: u8 = 0x01;
pub static ERR_STACK_TOO_SMALL: u8 = 0x02;
pub static ERR_NO_CREATE_PRIV: u8 = 0x03;

#[repr(C)]
struct Scheduler {
    // start fields used in assembly, do not change their order
    ptr_RT: usize, // pointer to running task
    ptr_HT: usize, // pointer to current high priority task (or the next task to be scheduled)
    // end fields used in assembly
    RT: usize,
    is_running: bool,
    threads: [TaskControlBlock; 32],
    ATV: [bool; 32],
    BTV: [bool; 32],
}

/// A single thread's state
#[repr(C)]
#[derive(Clone, Copy)]
struct TaskControlBlock {
    // fields used in assembly, do not reorder them
    sp: u32, // current stack pointer of this thread
    privileged: u32, // make it a word, assembly is easier. FIXME
}

// GLOBALS:
#[no_mangle]
static mut __CORTEXM_THREADS_GLOBAL_PTR: u32 = 0;
static mut __CORTEXM_THREADS_GLOBAL: Scheduler = Scheduler {
    ptr_RT: 0,
    ptr_HT: 0,
    RT: 0,
    is_running: false,
    threads: [TaskControlBlock {
        sp: 0,
        privileged: 0,
    }; 32],
    ATV: [false; 32],
    BTV: [false; 32],
};
// end GLOBALS

/// Initialize the switcher system
pub fn init() {
    unsafe {
        disable_interrupt();
        let ptr: usize = core::intrinsics::transmute(&__CORTEXM_THREADS_GLOBAL);
        __CORTEXM_THREADS_GLOBAL_PTR = ptr as u32;
        __CORTEXM_THREADS_GLOBAL.is_running = true;
        enable_interrupt();
    }
        /// The below section just sets up the timer and starts it.
        let cp = cortex_m::Peripherals::take().unwrap();
        let mut syst = cp.SYST;
        syst.set_clock_source(SystClkSource::Core);
        syst.set_reload(80_000);
        syst.enable_counter();
        syst.enable_interrupt();
}

pub fn release(task_ids: &[usize]) {
    let handler = unsafe { &mut __CORTEXM_THREADS_GLOBAL };
    task_ids.iter().for_each(|tid| {
        unsafe {
            handler.ATV[*tid] = true;
        }
    });
    SysTick();
}

pub fn create_task(
    priority: usize,
    stack: &mut [u32],
    handler_fn: fn() -> !,
) -> Result<(), u8> {
    unsafe {
        disable_interrupt();
        let handler = &mut __CORTEXM_THREADS_GLOBAL;

        match create_tcb(stack, handler_fn,true) {
            Ok(tcb) => {
                insert_tcb(priority, tcb);
            }
            Err(e) => {
                enable_interrupt();
                return Err(e);
            }
        }
        enable_interrupt();
        Ok(())
    }
}

fn preempt() {
    let handler = unsafe { &mut __CORTEXM_THREADS_GLOBAL };
    if handler.is_running {
        let HT = get_HT();
        // schedule a thread to be run
        if handler.RT != HT {
            handler.RT = HT;
            unsafe {
                handler.ptr_HT = core::intrinsics::transmute(&handler.threads[handler.RT]);
                // The following Causes PendSV interrupt, the interrupt handler is written in assembly
                let pend = ptr::read_volatile(0xE000ED04 as *const u32);
                ptr::write_volatile(0xE000ED04 as *mut u32, pend | 1 << 28);
            }
        }
    }
}

// SysTick Exception handler
#[no_mangle]
pub extern "C" fn SysTick() {
    unsafe {
        disable_interrupt();
    }
    preempt();
    unsafe {
        enable_interrupt();
    }
}

fn get_HT() -> usize {
    let handler = unsafe { &mut __CORTEXM_THREADS_GLOBAL };
    for i in (0..32).rev() {
        if handler.ATV[i] == true && handler.BTV[i] == false {
            return i;
        }
    };
    return 0;
}

fn create_tcb(
    stack: &mut [u32],
    handler: fn() -> !,
    priviliged: bool,
) -> Result<TaskControlBlock, u8> {
    if stack.len() < 32 {
        return Err(ERR_STACK_TOO_SMALL);
    }
    let idx = stack.len() - 1;
    stack[idx] = 1 << 24; // xPSR
    let pc: usize = unsafe { core::intrinsics::transmute(handler as *const fn()) };
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
    unsafe {
        let sp: usize = core::intrinsics::transmute(&stack[stack.len() - 16]);
        let tcb = TaskControlBlock {
            sp: sp as u32,
            privileged: if priviliged { 0x1 } else { 0x0 },
        };
        Ok(tcb)
    }
}

fn insert_tcb(idx: usize, tcb: TaskControlBlock) {
    unsafe {
        let handler = &mut __CORTEXM_THREADS_GLOBAL;
        handler.threads[idx] = tcb;
    }
}

pub fn block_unblock(tid: usize, flag: bool) {
    let handler = unsafe { &mut __CORTEXM_THREADS_GLOBAL };
    handler.BTV[tid] = flag;
}