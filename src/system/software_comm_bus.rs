use crate::config::MESSAGE_COUNT;
use crate::system::software_sync_bus::SemaphoreControlBlock;
use crate::KernelError;

use crate::system::types::{MessageId, SemaphoreId, TaskId};
use crate::types::BooleanVector;

#[derive(Clone, Copy)]
pub struct MessageControlBlock {
    pub receivers: BooleanVector,
}

pub struct MessagingManager {
    pub message_table: [Option<MessageControlBlock>; MESSAGE_COUNT],
    pub semaphore_table: SemaphoresTable,
}

pub struct SemaphoresTable {
    table: [Option<SemaphoreControlBlock>; MESSAGE_COUNT],
    curr: usize,
}

impl SemaphoresTable {
    pub const fn new() -> Self {
        Self {
            table: [None; MESSAGE_COUNT],
            curr: 0,
        }
    }
    pub fn add_semaphore(&mut self, task_mask: BooleanVector) -> Result<SemaphoreId, KernelError> {
        if self.curr >= MESSAGE_COUNT {
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
        tasks_mask: BooleanVector,
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

impl<'a> MessagingManager {
    pub const fn new() -> Self {
        Self {
            message_table: [None; MESSAGE_COUNT],
            semaphore_table: SemaphoresTable::new(),
        }
    }

    pub fn broadcast(&mut self, msg_id: MessageId) -> Result<u32, KernelError> {
        if self.message_table.get(msg_id).is_none() {
            return Err(KernelError::NotFound);
        }
        let mcb = self.message_table[msg_id].unwrap();
        let mask = self
            .semaphore_table
            .signal_and_release(msg_id, mcb.receivers)?;
        return Ok(mask);
    }

    pub fn receive(&'a mut self, msg_id: MessageId, curr_pid: TaskId) -> bool {
        match self.semaphore_table.test_and_reset(msg_id, curr_pid as u32) {
            Ok(res) if res == true => true,
            _ => false,
        }
    }

    pub fn create(
        &mut self,
        tasks_mask: BooleanVector,
        receivers_mask: BooleanVector,
    ) -> Result<MessageId, KernelError> {
        let msg_id = self.semaphore_table.add_semaphore(tasks_mask)?;
        self.message_table[msg_id].replace(MessageControlBlock {
            receivers: receivers_mask,
        });
        return Ok(msg_id);
    }
}
