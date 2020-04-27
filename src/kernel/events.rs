//! # Event Management Module
//!
//! Defines Kernel routines for Event Management.

use core::cell::RefCell;

use crate::utils::arch::{critical_section,Mutex};

use crate::priv_execute;
use crate::system::event::*;
use crate::utils::arch::is_privileged;
use crate::KernelError;
use crate::kernel::timer::get_time;

/// Global Instance of EventManager
static event_manager: Mutex<RefCell<EventTable>> = Mutex::new(RefCell::new(EventTable::new()));

/// Sweeps all events in event table and updates their counter, if counter has expired
/// then it dispatches the event and resets the counter.
pub fn sweep_event_table() {
    critical_section(|cs_token| {
        event_manager
            .borrow(cs_token)
            .borrow_mut()
            .sweep(get_time());
    })
}

/// This function is used to enable events.
pub fn enable(event_id: EventId) -> Result<(),KernelError> {
    critical_section(|cs_token| {
        event_manager
            .borrow(cs_token)
            .borrow_mut()
            .enable(event_id)
    })
}
/// This function is used to disable events.
pub fn disable(event_id: EventId) -> Result<(),KernelError> {
    critical_section(|cs_token| {
        event_manager
            .borrow(cs_token)
            .borrow_mut()
            .disable(event_id)
    })
}

/// Creates new Events.
pub fn new(
    is_enabled: bool,
    threshold: u32,
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