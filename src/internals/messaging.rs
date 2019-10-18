#![feature(const_fn)]
//use core::alloc::
use crate::config::{MAX_BUFFER_SIZE, MAX_TASKS, SEMAPHORE_COUNT};
use crate::errors::KernelError;
use crate::internals::semaphores::{SemaphoreControlBlock, SemaphoresTable};

use cortex_m_semihosting::hprintln;

use crate::internals::types::MessageId;

#[derive(Clone, Copy)]
pub struct MCB {
    pub receivers: u32,
}

#[derive(Clone, Copy)]
pub struct MessagingManager {
    pub mcb_table: [MCB; SEMAPHORE_COUNT],
    pub msg_scb_table: SemaphoresTable,
}

impl<'a> MessagingManager {
    pub const fn new() -> Self {
        Self {
            mcb_table: [MCB { receivers: 0 }; SEMAPHORE_COUNT],
            msg_scb_table: SemaphoresTable::new(),
        }
    }

    pub fn broadcast(&mut self, msg_id: MessageId) -> Result<u32, KernelError> {
        if self.mcb_table.get(msg_id).is_none() {
            return Err(KernelError::NotFound);
        }
        let mcb = self.mcb_table[msg_id];
        let mask = self
            .msg_scb_table
            .signal_and_release(msg_id, &mcb.receivers)?;
        return Ok(mask);
    }

    pub fn receive(&'a mut self, msg_id: MessageId, curr_pid: usize) -> bool {
        match self.msg_scb_table.test_and_reset(msg_id, curr_pid as u32) {
            Ok(res) if res == true => true,
            _ => false,
        }
    }

    pub fn create(
        &mut self,
        tasks_mask: u32,
        receivers_mask: u32,
    ) -> Result<MessageId, KernelError> {
        let msg_id = self.msg_scb_table.create(tasks_mask)?;
        self.mcb_table[msg_id].receivers |= receivers_mask;
        return Ok(msg_id);
    }
}
