//! # Machine specific
//!
//! Defines functions which are defined majorly in assembly. Thus, might change for one board to another.

// Platform specific Exports
pub use cortex_m::interrupt::free as critical_section;
pub use cortex_m::interrupt::Mutex;
pub use cortex_m::peripheral::syst::SystClkSource;
pub use cortex_m::peripheral::Peripherals;

use cortex_m_rt::exception;
use cortex_m::register::control;

use crate::kernel::tasks::{TaskManager,schedule};
use crate::system::scheduler::TaskControlBlock;

#[cfg(any(feature = "events_32", feature = "events_16", feature = "events_64"))]
use crate::kernel::events::sweep_event_table;

#[cfg(feature="process_monitor")]
use crate::kernel::task_monitor::sweep_deadlines;

#[cfg(feature="timer")]
use crate::kernel::timer::update_time;

/// Returns the MSB of `val`. It is written using CLZ instruction.
pub fn get_msb(val: u32) -> Option<usize> {
    let mut res: usize;
    unsafe {
        asm!(
            "clz {1}, {0}",
            in(reg) val,
            out(reg) res,
        );
    }
    res = 32 - res;
    if res == 0 {
        return None
    } else {
        res -= 1;
    }
    return Some(res);
}

/// Creates an SVC Interrupt
pub fn svc_call() {
    unsafe {
        asm!("svc 1");
    }
}

#[inline(always)]
pub unsafe fn return_to_psp() {
        asm!("
        ldr r0, =0xFFFFFFFD
        bx	r0
        ");
}

#[inline(always)]
pub fn save_context(task_stack: &TaskControlBlock) {
    unsafe {
        asm!(
            "mrs r0, psp",
            "subs r0, #16",
            "stmia r0!,{{r4-r7}}",
            "mov	r4, r8",
            "mov	r5, r9",
            "mov	r6, r10",
            "mov	r7, r11",
            "subs	r0, #32",
            "stmia	r0!,{{r4-r7}}",
            "subs	r0, #16",
            "mov	r1, {0}",
            "@ldr	r1, [r2]",
            "str	r0, [r1]",
            in(reg) task_stack,
            out("r0") _, 
            out("r1") _,
        )
    };
}

#[inline(always)]
pub fn load_context(task_stack: &TaskControlBlock) {
    unsafe {
        asm!(
            "cpsid	i",
            "mov	r1, {0}",
            "@ldr	r1, [r2]",
            "@ldr	r1, [r1]",
            "ldr	r0, [r1]",
            "ldmia	r0!,{{r4-r7}}",
            "mov	r8, r4",
            "mov	r9, r5",
            "mov	r10, r6",
            "mov	r11, r7",
            "ldmia	r0!,{{r4-r7}}",
            "msr	psp, r0",
            in(reg) task_stack,
            out("r0") _, 
            out("r1") _,
        )
    };
}

/// ### SysTick Interrupt handler
/// Its the Crux of the Kernel’s time management module and Task scheduling.
/// This interrupt handler updates the time and also dispatches the appropriate event handlers.
/// The interrupt handler also calls `schedule()` in here so as to dispatch any higher priority
/// task if there are any.

#[cfg(feature="timer")]
#[exception]
fn SysTick() {

    #[cfg(any(feature = "events_32", feature = "events_16", feature = "events_64"))]
    sweep_event_table();

    #[cfg(feature="timer")]
    update_time();
    
    #[cfg(feature="process_monitor")]
    sweep_deadlines();
    
    // hprintln!("hello");
    schedule();
}
/// ### SVC Interrupt handler,
/// calls `tasks::schedule()`
#[exception]
fn SVCall() {
    schedule();
}
/// ### PendSV Interrupt handler,
/// PendSV interrupt handler does the actual context switch in the Kernel.
#[exception]
fn PendSV() {
    critical_section(|cs_token| {
        let handler = &mut TaskManager.borrow(cs_token).borrow_mut();
        let curr_tid: usize = handler.curr_tid;
        let next_tid: usize = handler.get_next_tid() as usize;
        if curr_tid != next_tid || (!handler.started) {
            if handler.started {
                let curr_task = handler.task_control_blocks[curr_tid].as_ref().unwrap();
                curr_task.save_context();
            } else {
                handler.started = true;
            }
            let next_task = handler.task_control_blocks[next_tid].as_ref().unwrap();
            next_task.load_context();
    
            handler.curr_tid = next_tid;
        }
    });
    unsafe {return_to_psp()}
}

pub fn set_pendsv() {
    cortex_m::peripheral::SCB::set_pendsv();
}

pub fn wait_for_interrupt() {
    cortex_m::asm::wfi();
}

/// Returns true if Currently the Kernel is operating in Privileged mode.
pub fn is_privileged() -> bool {
    return control::read().npriv() == control::Npriv::Privileged
}