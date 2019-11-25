//! # Software synchronization bus definition
//!

use crate::config::SEMAPHORE_COUNT;
use crate::system::types::{SemaphoreId, TaskId};
use crate::types::BooleanVector;
use crate::KernelError;

/// Semaphores form the core of synchronization and communication in the Kernel.
#[derive(Clone, Copy)]
pub struct SemaphoreControlBlock {
    /// It is a boolean vector which represents the tasks notified by the semaphore.
    pub flags: BooleanVector,
    /// It is a boolean vector that corresponds to the tasks that are to be released by the semaphore on being signaled.
    pub tasks: BooleanVector,
}

/// Maintains state of all Semaphores in the Kernel.
pub struct SemaphoresTable {
    /// List of SemaphoreControlBlocks.
    table: [Option<SemaphoreControlBlock>; SEMAPHORE_COUNT],
    /// Max index in `table` till which `SemaphoreControlBlocks` have been allotted.
    curr: usize,
}


impl SemaphoreControlBlock {
    /// Creates and returns a new semaphore instance with tasks field set to `tasks_mask`.
    pub fn new(tasks: BooleanVector) -> Self {
        Self { flags: 0, tasks }
    }

    /// This method, when called, appends the `tasks_mask` to the flags field. Next, the tasks in the tasks field are released.
    pub fn signal_and_release(&mut self, tasks_mask: BooleanVector) -> Result<u32, KernelError> {
        self.flags |= tasks_mask;
        return Ok(self.tasks);
    }

    /// This method, when called, appends the `tasks_mask` to the `flags` field. Next, the `tasks` in the `tasks` field are released.
    pub fn test_and_reset(&mut self, curr_tid: TaskId) -> Result<bool, KernelError> {
        let curr_tid_mask = 1 << curr_tid;
        if self.flags & curr_tid_mask == curr_tid_mask {
            self.flags &= !curr_tid_mask;
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}

impl SemaphoresTable {
    /// Returns a new instance of a `SemaphoresTable`
    pub const fn new() -> Self {
        Self {
            table: [None; SEMAPHORE_COUNT],
            curr: 0,
        }
    }

    /// This method creates a new `SemaphoreControlBlock` with the `tasks` field of the semaphore as `tasks_mask` and adds it to the table field. It returns the id of the newly created semaphore.
    pub fn add_semaphore(&mut self, task_mask: BooleanVector) -> Result<SemaphoreId, KernelError> {
        if self.curr >= SEMAPHORE_COUNT {
            return Err(KernelError::LimitExceeded);
        }
        let id = self.curr;
        self.curr += 1;
        self.table[id].replace(SemaphoreControlBlock::new(task_mask));
        Ok(id)
    }

    /// Calls the `signal_and_release()` method on the semaphore with SemaphoreId as `sem_id` in the `semaphore_table`.
    pub fn signal_and_release(
        &mut self,
        sem_id: SemaphoreId,
        tasks_mask: BooleanVector,
    ) -> Result<u32, KernelError> {
        if let Some(sem) = &mut self.table[sem_id] {
            sem.signal_and_release(tasks_mask)
        } else {
            Err(KernelError::NotFound)
        }
    }

    /// Calls the `test_and_reset()` method on the semaphore with SemaphoreId as `sem_id` in the `semaphore_table`.
    pub fn test_and_reset(
        &mut self,
        sem_id: SemaphoreId,
        curr_pid: TaskId,
    ) -> Result<bool, KernelError> {
        if let Some(sem) = &mut self.table[sem_id] {
            sem.test_and_reset(curr_pid)
        } else {
            Err(KernelError::NotFound)
        }
    }
}
