use crate::internals::event_manager::*;
use crate::internals::helper::check_priv;
use crate::internals::types::{EventId, MessageId, SemaphoreId};
use crate::process::release;
use crate::KernelError;
use core::borrow::BorrowMut;
use core::cell::RefCell;
use cortex_m::interrupt::free as execute_critical;
use cortex_m::interrupt::Mutex;
use cortex_m::register::control::Npriv;
use cortex_m_semihosting::hprintln;
use crate::priv_execute;

pub use crate::internals::event_manager::{EventTableType, EventType};

static event_manager: Mutex<RefCell<EventManager>> = Mutex::new(RefCell::new(EventManager::new()));

pub fn sweep_event_table(event_type: EventTableType) {
    execute_critical(|cs_token| {
        event_manager
            .borrow(cs_token)
            .borrow_mut()
            .sweep(event_type);
    })
}

pub fn dispatch_event(event_id: EventId) {
    execute_critical(|cs_token| {
        event_manager
            .borrow(cs_token)
            .borrow_mut()
            .execute_event(event_id);
    })
}

pub fn enable_event(event_id: EventId) {
    execute_critical(|cs_token| {
        event_manager
            .borrow(cs_token)
            .borrow_mut()
            .enable_event(event_id);
    })
}

pub fn create_FreeRunning(
    is_enabled: bool,
    threshold: u8,
    event_counter_type: EventTableType,
) -> Result<EventId, KernelError> {
    priv_execute!({execute_critical(|cs_token| {
            Ok(event_manager.borrow(cs_token).borrow_mut().create(
                is_enabled,
                EventType::FreeRunning,
                threshold,
                event_counter_type,
            ))
        })})
}

pub fn create_OnOff(
    is_enabled: bool,
) -> Result<EventId, KernelError> {
    priv_execute!({execute_critical(|cs_token| {Ok(event_manager.borrow(cs_token).borrow_mut().create(
                is_enabled,
                EventType::OnOff,
                10,
                EventTableType::OnOff,
            ))})})
}

pub fn set_semaphore(event_id: EventId, sem: SemaphoreId, tasks_mask: u32) -> Result<(), KernelError> {
    priv_execute!({execute_critical(|cs_token| {
                event_manager
                    .borrow(cs_token)
                    .borrow_mut()
                    .set_semaphore(event_id, sem, tasks_mask)
            });
            Ok(())})

}

pub fn set_tasks(event_id: EventId, tasks: u32) -> Result<(), KernelError> {
    priv_execute!({execute_critical(|cs_token| {
                event_manager
                    .borrow(cs_token)
                    .borrow_mut()
                    .set_tasks(event_id, tasks)
            });
            Ok(())})
}

pub fn set_msg(event_id: EventId, msg_id: usize) -> Result<(), KernelError> {
    priv_execute!({execute_critical(|cs_token| {
                event_manager
                    .borrow(cs_token)
                    .borrow_mut()
                    .set_msg(event_id, msg_id)
            });
            Ok(())})
}

pub fn set_next_event(event_id: EventId, next: EventId) -> Result<(), KernelError> {
    priv_execute!({execute_critical(|cs_token| {
                event_manager
                    .borrow(cs_token)
                    .borrow_mut()
                    .set_next_event(event_id, next)
            });
            Ok(())})
}
