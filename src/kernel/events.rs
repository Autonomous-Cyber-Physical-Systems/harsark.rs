//! # Event Management Module
//!
//! Defines Kernel routines for Event Management.

use core::cell::RefCell;

use crate::utils::arch::{critical_section,Mutex};

use crate::priv_execute;
use crate::system::event::*;
use crate::utils::helpers::is_privileged;
use crate::KernelError;
use crate::kernel::timer::get_time;
/// Global Instance of EventManager
static event_manager: Mutex<RefCell<EventTable>> = Mutex::new(RefCell::new(EventTable::new()));

/// Dispatches all the events of EventTableType same as `event_type`.
pub fn sweep_event_table() {
    critical_section(|cs_token| {
        event_manager
            .borrow(cs_token)
            .borrow_mut()
            .sweep(get_time());
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




/// Creates a new Event of type EventType::FreeRunning.
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