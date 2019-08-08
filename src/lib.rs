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
    inited: bool,
    idx: usize,
    add_idx: usize,
    threads: [ThreadControlBlock; 32],
}

/// Thread status
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum ThreadStatus {
    Idle,
    Sleeping,
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
    priority: u8,
    status: ThreadStatus,
    sleep_ticks: u32,
}

// GLOBALS:
#[no_mangle]
static mut __CORTEXM_THREADS_GLOBAL_PTR: u32 = 0;
static mut __CORTEXM_THREADS_GLOBAL: ThreadsState = ThreadsState {
    ptr_RT: 0,
    ptr_HT: 0,
    inited: false,
    idx: 0,
    add_idx: 1,
    threads: [ThreadControlBlock {
        sp: 0,
        status: ThreadStatus::Idle,
        priority: 0,
        privileged: 0,
        sleep_ticks: 0,
    }; 32],
};
// end GLOBALS

// functions defined in assembly
extern "C" {
    fn __CORTEXM_THREADS_cpsid();
    fn __CORTEXM_THREADS_cpsie();
    fn __CORTEXM_THREADS_wfe();
}

/// Initialize the switcher system
pub fn init() -> ! {
    unsafe {
        __CORTEXM_THREADS_cpsid();
        let ptr: usize = core::intrinsics::transmute(&__CORTEXM_THREADS_GLOBAL);
        __CORTEXM_THREADS_GLOBAL_PTR = ptr as u32;
        __CORTEXM_THREADS_cpsie();
        let mut idle_stack = [0xDEADBEEF; 64];
        match create_tcb(
            &mut idle_stack,
            || loop {
                __CORTEXM_THREADS_wfe();
            },
            0xff,
            false,
        ) {
            Ok(tcb) => {
                insert_tcb(0, tcb);
            }
            _ => panic!("Could not create idle thread"),
        }
        __CORTEXM_THREADS_GLOBAL.inited = true;
        SysTick();
        loop {
            __CORTEXM_THREADS_wfe();
        }
    }
}

pub fn create_thread_with_config(
    stack: &mut [u32],
    handler_fn: fn() -> !,
    priority: u8,
) -> Result<(), u8> {
    unsafe {
        __CORTEXM_THREADS_cpsid();
        let handler = &mut __CORTEXM_THREADS_GLOBAL;
        if handler.add_idx >= handler.threads.len() {
            return Err(ERR_TOO_MANY_THREADS);
        }
        if handler.inited && handler.threads[handler.idx].privileged == 0 {
            return Err(ERR_NO_CREATE_PRIV);
        }
        match create_tcb(stack, handler_fn, priority, true) {
            Ok(tcb) => {
                insert_tcb(handler.add_idx, tcb);
                handler.add_idx = handler.add_idx + 1;
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
            handler.idx = get_next_thread_idx();
            unsafe {
                handler.ptr_HT = core::intrinsics::transmute(&handler.threads[handler.idx]);
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

/// Get id of current thread
pub fn get_thread_id() -> usize {
    let handler = unsafe { &mut __CORTEXM_THREADS_GLOBAL };
    handler.idx
}

pub fn sleep(ticks: u32) {
    let handler = unsafe { &mut __CORTEXM_THREADS_GLOBAL };
    if handler.idx > 0 {
        handler.threads[handler.idx].status = ThreadStatus::Sleeping;
        handler.threads[handler.idx].sleep_ticks = ticks;
        // schedule another thread
        SysTick();
    }
}

fn get_next_thread_idx() -> usize {
    let handler = unsafe { &mut __CORTEXM_THREADS_GLOBAL };
    if handler.add_idx <= 1 {
        // no user threads, schedule idle thread
        return 0;
    }
    // user threads exist
    // update sleeping threads
    for i in 1..handler.add_idx {
        if handler.threads[i].status == ThreadStatus::Sleeping {
            if handler.threads[i].sleep_ticks > 0 {
                handler.threads[i].sleep_ticks = handler.threads[i].sleep_ticks - 1;
            } else {
                handler.threads[i].status = ThreadStatus::Idle;
            }
        }
    }
    match handler
        .threads
        .into_iter()
        .enumerate()
        .filter(|&(idx, x)| idx > 0 && idx < handler.add_idx && x.status != ThreadStatus::Sleeping)
        .max_by(|&(i, _), &(j, _)| i.cmp(&j))
        {
            Some((idx, _)) => idx,
            _ => 0,
        }
}

fn create_tcb(
    stack: &mut [u32],
    handler: fn() -> !,
    priority: u8,
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
            priority: priority,
            privileged: if priviliged { 0x1 } else { 0x0 },
            status: ThreadStatus::Idle,
            sleep_ticks: 0,
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
