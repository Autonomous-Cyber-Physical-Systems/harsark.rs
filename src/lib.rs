#![no_std]

use core::ptr;

pub static ERR_TOO_MANY_THREADS: u8 = 0x01;
pub static ERR_STACK_TOO_SMALL: u8 = 0x02;
pub static ERR_NO_CREATE_PRIV: u8 = 0x03;

#[repr(C)]
struct ThreadsState {
    // start fields used in assembly, do not change their order
    ptr_RT: usize, // pointer to running task
    ptr_HT: usize, // pointer to current high priority task (or the next task to be scheduled)
    // end fields used in assembly
    RT: usize,
    inited: bool,
    threads: [ThreadControlBlock; 32],
    ATV: [bool; 32],
}

/// A single thread's state
#[repr(C)]
#[derive(Clone, Copy)]
struct ThreadControlBlock {
    // start fields used in assembly, do not reorder them
    /// current stack pointer of this thread
    sp: u32,
    privileged: u32, // make it a word, assembly is easier. FIXME
    // end fields used in assembly
}

// GLOBALS:
#[no_mangle]
static mut __CORTEXM_THREADS_GLOBAL_PTR: u32 = 0;
static mut __CORTEXM_THREADS_GLOBAL: ThreadsState = ThreadsState {
    ptr_RT: 0,
    ptr_HT: 0,
    RT: 0,
    inited: false,
    threads: [ThreadControlBlock {
        sp: 0,
        privileged: 0,
    }; 32],
    ATV: [false; 32]
};
// end GLOBALS

// functions defined in assembly
extern "C" {
    fn __CORTEXM_THREADS_cpsid();
    fn __CORTEXM_THREADS_cpsie();
    fn __CORTEXM_THREADS_wfe();
}

/// Initialize the switcher system
pub fn init() {
    unsafe {
        __CORTEXM_THREADS_cpsid();
        let ptr: usize = core::intrinsics::transmute(&__CORTEXM_THREADS_GLOBAL);
        __CORTEXM_THREADS_GLOBAL_PTR = ptr as u32;
        __CORTEXM_THREADS_cpsie();

        __CORTEXM_THREADS_GLOBAL.inited = true;
        SysTick();
    }
}

pub fn create_thread_with_config(
    stack: &mut [u32],
    handler_fn: fn() -> !,
    priority: usize,
) -> Result<(), u8> {
    unsafe {
        __CORTEXM_THREADS_cpsid();
        let handler = &mut __CORTEXM_THREADS_GLOBAL;

        match create_tcb(stack, handler_fn,true) {
            Ok(tcb) => {
                insert_tcb(priority, tcb);
            }
            Err(e) => {
                __CORTEXM_THREADS_cpsie();
                return Err(e);
            }
        }
        __CORTEXM_THREADS_cpsie();
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn SysTick() {
    unsafe {
        __CORTEXM_THREADS_cpsid();
    }
    let handler = unsafe { &mut __CORTEXM_THREADS_GLOBAL };
    if handler.inited {
        if handler.ptr_RT == handler.ptr_HT {
            // schedule a thread to be run
            handler.RT = get_next_thread_idx();
            unsafe {
                handler.ptr_HT = core::intrinsics::transmute(&handler.threads[handler.RT]);
            }
        }
        if handler.ptr_RT != handler.ptr_HT {
            unsafe {
                let pend = ptr::read_volatile(0xE000ED04 as *const u32);
                ptr::write_volatile(0xE000ED04 as *mut u32, pend | 1 << 28);
            }
        }
    }
    unsafe {
        __CORTEXM_THREADS_cpsie();
    }
}

static mut bleh : usize = 1;
fn get_next_thread_idx() -> usize {
//    let handler = unsafe { &mut __CORTEXM_THREADS_GLOBAL };
    unsafe {
    if bleh == 1 {
        bleh = 2;
        return 2;
    } else {
        bleh = 1;
        return 1;
    }
    }
}

fn create_tcb(
    stack: &mut [u32],
    handler: fn() -> !,
    priviliged: bool,
) -> Result<ThreadControlBlock, u8> {
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
        let tcb = ThreadControlBlock {
            sp: sp as u32,
            privileged: if priviliged { 0x1 } else { 0x0 },
        };
        Ok(tcb)
    }
}

fn insert_tcb(idx: usize, tcb: ThreadControlBlock) {
    unsafe {
        let handler = &mut __CORTEXM_THREADS_GLOBAL;
        handler.threads[idx] = tcb;
    }
}
