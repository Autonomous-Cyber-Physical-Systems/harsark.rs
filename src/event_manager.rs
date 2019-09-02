use crate::config::{EVENT_INDEX_TABLE_COUNT, EVENT_NO};
use crate::kernel::task_manager::release;
use crate::{messaging::*, sync::*};
use cortex_m::interrupt::free as execute_critical;
use cortex_m_semihosting::hprintln;
use crate::kernel::event_manager::*;
use core::borrow::{BorrowMut};
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

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
    //    event_manager.borrow(cs_token).borrow_mut().execute_event(event_id);
    })
}
