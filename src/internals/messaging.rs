#![feature(const_fn)]

use crate::config::{MESSAGE_COUNT};
use crate::errors::KernelError;
use crate::internals::semaphores::SemaphoreControlBlock;

use crate::internals::types::{MessageId, SemaphoreId};

#[derive(Clone, Copy)]
pub struct MCB {
    pub receivers: u32,
}

pub struct MessagingManager {
    pub mcb_table: [Option<MCB>; MESSAGE_COUNT],
    pub scb_table: SemaphoresTable,
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
    pub fn add_semaphore(&mut self, task_mask: u32) -> Result<SemaphoreId, KernelError> {
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
        tasks_mask: u32,
    ) -> Result<u32, KernelError> {
        if let Some(mut sem) = self.table[sem_id] {
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
        if let Some(mut sem) = self.table[sem_id] {
            sem.test_and_reset(curr_pid)
        } else {
            Err(KernelError::NotFound)
        }
    }
}

impl<'a> MessagingManager {
    pub const fn new() -> Self {
        Self {
            mcb_table: [None; MESSAGE_COUNT],
            scb_table: SemaphoresTable::new(),
        }
    }

    pub fn broadcast(&mut self, msg_id: MessageId) -> Result<u32, KernelError> {
        if self.mcb_table.get(msg_id).is_none() {
            return Err(KernelError::NotFound);
        }
        let mcb = self.mcb_table[msg_id].unwrap();
        let mask = self
            .scb_table
            .signal_and_release(msg_id, mcb.receivers)?;
        return Ok(mask);
    }

    pub fn receive(&'a mut self, msg_id: MessageId, curr_pid: usize) -> bool {
        match self.scb_table.test_and_reset(msg_id, curr_pid as u32) {
            Ok(res) if res == true => true,
            _ => false,
        }
    }

    pub fn create(
        &mut self,
        tasks_mask: u32,
        receivers_mask: u32,
    ) -> Result<MessageId, KernelError> {
        let msg_id = self.scb_table.add_semaphore(tasks_mask)?;
        self.mcb_table[msg_id].replace(MCB{
            receivers: receivers_mask,
        });
        return Ok(msg_id);
    }
}
