use crate::config::SEMAPHORE_COUNT;
use crate::errors::KernelError;
use crate::helper::generate_task_mask;
use crate::kernel::semaphores::*;
use cortex_m::interrupt::free as execute_critical;

use core::borrow::BorrowMut;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

pub use crate::kernel::semaphores::SemaphoreId;

lazy_static! {
    static ref SCB_table: Mutex<RefCell<Semaphores>> = Mutex::new(RefCell::new(Semaphores {
        table: [SCB { flags: 0, tasks: 0 }; SEMAPHORE_COUNT],
        curr: 0
    }));
}

pub fn sem_post(sem_id: SemaphoreId, tasks: &[u32]) -> Result<(), KernelError> {
    execute_critical(|cs_token| {
        SCB_table
            .borrow(cs_token)
            .borrow_mut()
            .signal_and_release(sem_id, &generate_task_mask(tasks))
    })
}

pub fn sem_wait(sem_id: SemaphoreId) -> Result<bool, KernelError> {
    execute_critical(|cs_token| {
        SCB_table
            .borrow(cs_token)
            .borrow_mut()
            .test_and_reset(sem_id)
    })
}

pub fn new(tasks: &[u32]) -> Result<SemaphoreId, KernelError> {
    execute_critical(|cs_token| SCB_table.borrow(cs_token).borrow_mut().new(tasks))
}
