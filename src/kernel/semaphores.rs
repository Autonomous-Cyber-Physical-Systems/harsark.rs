use crate::config::SEMAPHORE_COUNT;
use crate::errors::KernelError;
use crate::helper::generate_task_mask;
use crate::task_manager::{get_RT, release};
use cortex_m::interrupt::free as execute_critical;
use cortex_m_semihosting::hprintln;

use core::borrow::BorrowMut;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

pub type SemaphoreId = usize;

#[derive(Clone, Copy)]
pub struct SCB {
    pub flags: u32,
    pub tasks: u32,
}

pub struct Semaphores {
    pub table: [SCB; SEMAPHORE_COUNT],
    pub curr: usize,
}

impl SCB {
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
    pub fn new(&mut self, tasks: &[u32]) -> Result<SemaphoreId, KernelError> {
        execute_critical(|_| {
            if self.curr >= SEMAPHORE_COUNT {
                return Err(KernelError::LimitExceeded);
            }
            let id = self.curr;
            self.curr += 1;
            self.table[id].tasks = generate_task_mask(tasks);
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
