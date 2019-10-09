use crate::config::SEMAPHORE_COUNT;
use crate::errors::KernelError;
use crate::kernel::semaphores::*;
use cortex_m::interrupt::free as execute_critical;
use crate::kernel::helper::check_priv;
use cortex_m::register::control::Npriv;
use core::borrow::BorrowMut;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use crate::process::{get_pid, release};
pub use crate::kernel::semaphores;

use crate::kernel::types::SemaphoreId;

static SCB_table: Mutex<RefCell<Semaphores>> = Mutex::new(RefCell::new(Semaphores::new()));

pub fn sem_post(sem_id: SemaphoreId, tasks_mask: u32) -> Result<(), KernelError> {
    execute_critical(|cs_token| {
        let mask = SCB_table
            .borrow(cs_token)
            .borrow_mut()
            .signal_and_release(sem_id, &tasks_mask)?;
        release(&mask)
    })
}

pub fn sem_wait(sem_id: SemaphoreId) -> Result<bool, KernelError> {
    execute_critical(|cs_token| {
        SCB_table
            .borrow(cs_token)
            .borrow_mut()
            .test_and_reset(sem_id, get_pid() as u32)
    })
}

pub fn create(tasks_mask: u32) -> Result<SemaphoreId, KernelError> {
    match check_priv() {
        Npriv::Unprivileged => {
            Err(KernelError::AccessDenied)
        },
        Npriv::Privileged => {
            execute_critical(|cs_token| {
                SCB_table
                    .borrow(cs_token)
                    .borrow_mut()
                    .create(tasks_mask)
            })
        },
    }
}
