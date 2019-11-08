use crate::kernel::tasks::{get_curr_tid, release};
use crate::priv_execute;
use crate::system::software_sync_bus;
use crate::system::software_sync_bus::*;
use crate::utils::arch::is_privileged;
use crate::KernelError;
use core::borrow::BorrowMut;
use core::cell::RefCell;
use cortex_m::interrupt::free as execute_critical;
use cortex_m::interrupt::Mutex;
use cortex_m::register::control::Npriv;

use crate::system::types::SemaphoreId;

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
        execute_critical(|cs_token| {
            SCB_table
                .borrow(cs_token)
                .borrow_mut()
                .add_semaphore(tasks_mask)
        })
    })
}
