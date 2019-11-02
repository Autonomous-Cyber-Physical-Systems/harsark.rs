use crate::errors::KernelError;
use crate::internals::helper::check_priv;
pub use crate::internals::semaphores;
use crate::internals::semaphores::*;
use crate::priv_execute;
use crate::process::{get_curr_tid, release};
use core::borrow::BorrowMut;
use core::cell::RefCell;
use cortex_m::interrupt::free as execute_critical;
use cortex_m::interrupt::Mutex;
use cortex_m::register::control::Npriv;

use crate::internals::types::SemaphoreId;

static SCB_table: Mutex<RefCell<SemaphoresTable>> =
    Mutex::new(RefCell::new(SemaphoresTable::new()));

pub fn signal_and_release(sem_id: SemaphoreId, tasks_mask: u32) -> Result<(), KernelError> {
    execute_critical(|cs_token| {
        let mask = SCB_table
            .borrow(cs_token)
            .borrow_mut()
            .signal_and_release(sem_id, tasks_mask)?;
        release(mask);
        Ok(())
    })
}

pub fn test_and_reset(sem_id: SemaphoreId) -> Result<bool, KernelError> {
    execute_critical(|cs_token| {
        SCB_table
            .borrow(cs_token)
            .borrow_mut()
            .test_and_reset(sem_id, get_curr_tid() as u32)
    })
}

pub fn new(tasks_mask: u32) -> Result<SemaphoreId, KernelError> {
    priv_execute!({
        execute_critical(|cs_token| SCB_table.borrow(cs_token).borrow_mut().add_semaphore(tasks_mask))
    })
}
