//! # Software Synchronization Module
//!
//! This module instantiates a global instance of SemaphoreTable and then defines Kernel Routines
//! which handle task synchronization.

use core::cell::RefCell;

use cortex_m::interrupt::free as execute_critical;
use cortex_m::interrupt::Mutex;

use crate::kernel::task_management::{get_curr_tid, release};
use crate::priv_execute;
use crate::system::software_sync_bus::*;
use crate::utils::arch::is_privileged;
use crate::KernelError;

use crate::system::types::{BooleanVector, SemaphoreId};

/// Global instance of SemaphoresTable
static SCB_table: Mutex<RefCell<SemaphoresTable>> =
    Mutex::new(RefCell::new(SemaphoresTable::new()));

/// Calls the `signal_and_release` method on the `semaphores_table` with SemaphoreID as `sem_id`.
pub fn signal_and_release(
    sem_id: SemaphoreId,
    tasks_mask: BooleanVector,
) -> Result<(), KernelError> {
    execute_critical(|cs_token| {
        let mask = SCB_table
            .borrow(cs_token)
            .borrow_mut()
            .signal_and_release(sem_id, tasks_mask)?;
        release(mask);
        Ok(())
    })
}

/// Calls the `test_and_reset` method on the `semaphores_table` with SemaphoreID as `sem_id`.
pub fn test_and_reset(sem_id: SemaphoreId) -> Result<bool, KernelError> {
    execute_critical(|cs_token| {
        SCB_table
            .borrow(cs_token)
            .borrow_mut()
            .test_and_reset(sem_id, get_curr_tid() as u32)
    })
}

/// Calls the `add_semaphore` method on the `semaphores_table`, which creates a new semaphore and returns its SemaphoreID.
pub fn new(tasks_mask: BooleanVector) -> Result<SemaphoreId, KernelError> {
    priv_execute!({
        execute_critical(|cs_token| {
            SCB_table
                .borrow(cs_token)
                .borrow_mut()
                .add_semaphore(tasks_mask)
        })
    })
}
