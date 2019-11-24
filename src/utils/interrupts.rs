//! Interrupt Handlers
use cortex_m_rt::exception;

use crate::kernel::events::sweep_event_table;
use crate::kernel::tasks::{is_preemptive, schedule};
use crate::kernel::time::tick;
use crate::system::event_manager::EventTableType;
use crate::system::time_manager::TickType;
use crate::utils::arch::pendSV_handler;

/// ### SysTick Interrupt handler
/// Its the Crux of the Kernel’s time management module and Task scheduling.
/// This interrupt handler updates the time and also dispatches the appropriate event handlers.
/// The interrupt handler also calls `schedule()` in here so as to dispatch any higher priority
/// task if there are any.
#[exception]
fn SysTick() {
    if is_preemptive() {
        schedule();
    }

    sweep_event_table(EventTableType::OnOff);
    sweep_event_table(EventTableType::MilliSec);

    match tick() {
        TickType::Hour => {
            sweep_event_table(EventTableType::Sec);
            sweep_event_table(EventTableType::Min);
            sweep_event_table(EventTableType::Hour);
        }
        TickType::Min => {
            sweep_event_table(EventTableType::Sec);
            sweep_event_table(EventTableType::Min);
        }
        TickType::Sec => {
            sweep_event_table(EventTableType::Sec);
        }
        _ => {}
    }
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
