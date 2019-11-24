//~ # Event Management Module
//~
//~ Defines Kernel routines for Event Management.

use core::cell::RefCell;

use cortex_m::interrupt::free as execute_critical;
use cortex_m::interrupt::Mutex;

use crate::priv_execute;
use crate::system::event_manager::*;
use crate::system::types::{BooleanVector, EventId, MessageId, SemaphoreId};
use crate::utils::arch::is_privileged;
use crate::KernelError;

use crate::system::event_manager::{EventTableType, EventType};

/// Global Instance of EventManager
static event_manager: Mutex<RefCell<EventManager>> = Mutex::new(RefCell::new(EventManager::new()));

/// Dispatches all the events of EventTableType same as `event_type`.
pub fn sweep_event_table(event_type: EventTableType) {
    execute_critical(|cs_token| {
        event_manager
            .borrow(cs_token)
            .borrow_mut()
            .sweep(event_type);
    })
}

/// This function is used to enable events if disabled. Useful for dispatching OnOff type events.
pub fn enable_event(event_id: EventId) {
    execute_critical(|cs_token| {
        event_manager
            .borrow(cs_token)
            .borrow_mut()
            .enable_event(event_id);
    })
}

/// Creates a new Event of type EventType::FreeRunning.
pub fn new_FreeRunning(
    is_enabled: bool,
    threshold: u8,
    event_counter_type: EventTableType,
) -> Result<EventId, KernelError> {
    priv_execute!({
        execute_critical(|cs_token| {
            Ok(event_manager.borrow(cs_token).borrow_mut().create(
                is_enabled,
                EventType::FreeRunning,
                threshold,
                event_counter_type,
            )?)
        })
    })
}

/// Creates a new Event of type EventType::OnOff.
pub fn new_OnOff(is_enabled: bool) -> Result<EventId, KernelError> {
    priv_execute!({
        execute_critical(|cs_token| {
            Ok(event_manager.borrow(cs_token).borrow_mut().create(
                is_enabled,
                EventType::OnOff,
                0,
                EventTableType::OnOff,
            )?)
        })
    })
}

/// Configure the event with EventId as `event_id` for signalling a semaphore on dispatch.
pub fn set_semaphore(
    event_id: EventId,
    sem: SemaphoreId,
    tasks_mask: BooleanVector,
) -> Result<(), KernelError> {
    priv_execute!({
        execute_critical(|cs_token| {
            event_manager
                .borrow(cs_token)
                .borrow_mut()
                .set_semaphore(event_id, sem, tasks_mask)
        })
    })
}

/// Configure the event with EventId as event_id for releasing tasks in the `tasks_mask`.
pub fn set_tasks(event_id: EventId, tasks: BooleanVector) -> Result<(), KernelError> {
    priv_execute!({
        execute_critical(|cs_token| {
            event_manager
                .borrow(cs_token)
                .borrow_mut()
                .set_tasks(event_id, tasks)
        })
    })
}

/// Configure the event with EventId as `event_id` to broadcast the message with MessageId as `msg_id`.
pub fn set_message(event_id: EventId, msg_id: MessageId) -> Result<(), KernelError> {
    priv_execute!({
        execute_critical(|cs_token| {
            event_manager
                .borrow(cs_token)
                .borrow_mut()
                .set_message(event_id, msg_id)
        })
    })
}

/// Configure the event with EventId as `event_id` to dispatch the event with EventId as `next_event`.
pub fn set_next_event(event_id: EventId, next: EventId) -> Result<(), KernelError> {
    priv_execute!({
        execute_critical(|cs_token| {
            event_manager
                .borrow(cs_token)
                .borrow_mut()
                .set_next_event(event_id, next)
        })
    })
}
