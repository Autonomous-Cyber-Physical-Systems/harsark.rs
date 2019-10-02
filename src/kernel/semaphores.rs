use crate::config::SEMAPHORE_COUNT;
use crate::errors::KernelError;
use crate::kernel::helper::generate_task_mask;
use crate::process::{get_RT, release};
use cortex_m::interrupt::free as execute_critical;
use cortex_m_semihosting::hprintln;

use core::borrow::BorrowMut;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use crate::kernel::types::SemaphoreId;

#[derive(Clone, Copy)]
pub struct SemaphoreControlBlock {
    flags: u32,
    tasks: u32,
}

#[derive(Clone, Copy)]
pub struct Semaphores {
    table: [SemaphoreControlBlock; SEMAPHORE_COUNT],
    curr: usize,
}

impl SemaphoreControlBlock {
    pub fn signal_and_release(&mut self, tasks_mask: &u32) -> Result<(), KernelError> {
        execute_critical(|_| {
            self.flags |= *tasks_mask;
            release(&self.tasks);
            return Ok(());
        })
    }
    pub fn test_and_reset(&mut self) -> Result<bool, KernelError> {
        execute_critical(|_| {
            let rt = get_RT() as u32;
            let rt_mask = (1 << rt);
            if self.flags & rt_mask == rt_mask {
                self.flags &= !rt_mask;
                return Ok(true);
            } else {
                return Ok(false);
            }
        })
    }
}

impl Semaphores {
    pub const fn new() -> Self {
        Self {
            table: [SemaphoreControlBlock { flags: 0, tasks: 0 }; SEMAPHORE_COUNT],
            curr: 0,
        }
    }
    pub fn create(&mut self, task_mask: u32) -> Result<SemaphoreId, KernelError> {
        execute_critical(|_| {
            if self.curr >= SEMAPHORE_COUNT {
                return Err(KernelError::LimitExceeded);
            }
            let id = self.curr;
            self.curr += 1;
            self.table[id].tasks = task_mask;
            Ok(id)
        })
    }

    pub fn signal_and_release(
        &mut self,
        sem_id: SemaphoreId,
        tasks_mask: &u32,
    ) -> Result<(), KernelError> {
        self.table[sem_id].signal_and_release(tasks_mask)
    }

    pub fn test_and_reset(&mut self, sem_id: SemaphoreId) -> Result<bool, KernelError> {
        self.table[sem_id].test_and_reset()
    }
}
