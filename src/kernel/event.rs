//! # Event Management Module
//!
//! Defines Kernel routines for Event Management.

use core::cell::RefCell;

use crate::utils::arch::{critical_section,Mutex,SystClkSource,Peripherals};

use crate::priv_execute;
use crate::system::event::*;
use crate::utils::helpers::is_privileged;
use crate::KernelError;

/// Global Instance of EventManager
static event_manager: Mutex<RefCell<EventTable>> = Mutex::new(RefCell::new(EventTable::new()));

/// Dispatches all the events of EventTableType same as `event_type`.
pub fn sweep_event_table() {
    critical_section(|cs_token| {
        event_manager
            .borrow(cs_token)
            .borrow_mut()
            .sweep();
    })
}

/// This function is used to enable events if disabled. Useful for dispatching OnOff type events.
pub fn enable(event_id: EventId) -> Result<(),KernelError> {
    critical_section(|cs_token| {
        event_manager
            .borrow(cs_token)
            .borrow_mut()
            .enable(event_id)
    })
}
/// This function is used to enable events if disabled. Useful for dispatching OnOff type events.
pub fn disable(event_id: EventId) -> Result<(),KernelError> {
    critical_section(|cs_token| {
        event_manager
            .borrow(cs_token)
            .borrow_mut()
            .disable(event_id)
    })
}


/// Starts the Kernel scheduler, which starts scheduling tasks and starts the SysTick timer using the
/// reference of the Peripherals instance and the `tick_interval`. `tick_interval` specifies the
/// frequency of the timer interrupt. The SysTick exception updates the kernel regarding the time
/// elapsed, which is used to dispatch events and schedule tasks.
pub fn start_timer(peripherals: &mut Peripherals, tick_interval: u32) {
    let syst = &mut peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(tick_interval);
    syst.enable_counter();
    syst.enable_interrupt();
}

/// Creates a new Event of type EventType::FreeRunning.
pub fn new(
    is_enabled: bool,
    threshold: u8,
    handler: fn() -> (),
) -> Result<EventId, KernelError> {
    priv_execute!({
        critical_section(|cs_token| {
            event_manager.borrow(cs_token).borrow_mut().create(
                is_enabled,
                threshold,
                handler,
            )
        })
    })
}