use crate::config::SEMAPHORE_COUNT;
use crate::errors::KernelError;
use crate::task_manager::{get_RT, release};
use cortex_m::interrupt::free as execute_critical;
use crate::generate_task_mask;

pub type SemaphoreId = usize;

#[derive(Clone, Copy)]
pub struct SCB {
    pub flags: u32,
    pub tasks: u32,
}

pub struct Semaphores {
    table: [SCB; SEMAPHORE_COUNT],
    curr: usize
}

static mut SCB_TABLE: Semaphores = Semaphores {
    table: [ SCB { flags: 0, tasks: 0}; SEMAPHORE_COUNT],
    curr: 0
};

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
    pub fn new(tasks: &[u32]) -> Result<SemaphoreId, KernelError> {
        execute_critical(|_| {
            let scb_table = unsafe {&mut SCB_TABLE};
            if scb_table.curr >= SEMAPHORE_COUNT {
                return Err(KernelError::LimitExceeded)
            }
            let id = scb_table.curr;
            scb_table.curr += 1;
            scb_table.table[id].tasks = generate_task_mask(tasks);
            Ok(id)
        })
    }

    pub fn signal_and_release(&mut self, sem_id: SemaphoreId, tasks_mask: &u32) -> Result<(), KernelError> {
        self.table[sem_id].signal_and_release(tasks_mask)
    }

    pub fn test_and_reset(&mut self, sem_id: SemaphoreId) -> Result<bool, KernelError> {
        self.table[sem_id].test_and_reset()
    }
}
