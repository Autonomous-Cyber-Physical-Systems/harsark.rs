//! Interrupt Handlers
use cortex_m_rt::exception;

use crate::kernel::event::sweep_event_table;
use crate::kernel::task::schedule;
use crate::utils::arch::pendSV_handler;

/// ### SysTick Interrupt handler
/// Its the Crux of the Kernel’s time management module and Task scheduling.
/// This interrupt handler updates the time and also dispatches the appropriate event handlers.
/// The interrupt handler also calls `schedule()` in here so as to dispatch any higher priority
/// task if there are any.
#[exception]
fn SysTick() {
    sweep_event_table();
    schedule();
}

/// ### SVC Interrupt handler,
/// calls `tasks::schedule()`
#[exception]
fn SVCall() {
    schedule();
}

/// ### PendSV Interrupt handler,
/// calls `utils::arch::pendSV_handler()`
#[exception]
fn PendSV() {
    pendSV_handler();
}
