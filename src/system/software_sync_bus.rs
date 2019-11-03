use crate::config::SEMAPHORE_COUNT;
use crate::KernelError;
use crate::system::types::SemaphoreId;
use cortex_m_semihosting::hprintln;

#[derive(Clone, Copy)]
pub struct SemaphoreControlBlock {
    pub flags: u32,
    pub tasks: u32,
}

pub struct SemaphoresTable {
    table: [Option<SemaphoreControlBlock>; SEMAPHORE_COUNT],
    curr: usize,
}

impl SemaphoreControlBlock {
    pub fn new(tasks: u32) -> Self {
        Self {
            flags: 0,
            tasks
        }
    }
    pub fn signal_and_release(&mut self, tasks_mask: u32) -> Result<u32, KernelError> {
        self.flags |= tasks_mask;
        return Ok(self.tasks);
    }
    pub fn test_and_reset(&mut self, curr_pid: u32) -> Result<bool, KernelError> {
        let curr_pid_mask = 1 << curr_pid;
        if self.flags & curr_pid_mask == curr_pid_mask {
            self.flags &= !curr_pid_mask;
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}

impl SemaphoresTable {
    pub const fn new() -> Self {
        Self {
            table: [None; SEMAPHORE_COUNT],
            curr: 0,
        }
    }
    pub fn add_semaphore(&mut self, task_mask: u32) -> Result<SemaphoreId, KernelError> {
        if self.curr >= SEMAPHORE_COUNT {
            return Err(KernelError::LimitExceeded);
        }
        let id = self.curr;
        self.curr += 1;
        self.table[id].replace(SemaphoreControlBlock::new(task_mask));
        Ok(id)
    }

    pub fn signal_and_release(
        &mut self,
        sem_id: SemaphoreId,
        tasks_mask: u32,
    ) -> Result<u32, KernelError> {
        if let Some(sem) = &mut self.table[sem_id] {
            sem.signal_and_release(tasks_mask)
        } else {
            Err(KernelError::NotFound)
        }
    }

    pub fn test_and_reset(
        &mut self,
        sem_id: SemaphoreId,
        curr_pid: u32,
    ) -> Result<bool, KernelError> {
        if let Some(sem) = &mut self.table[sem_id] {
            sem.test_and_reset(curr_pid)
        } else {
            Err(KernelError::NotFound)
        }
    }
}
