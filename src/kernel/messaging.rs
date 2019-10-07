#![feature(const_fn)]
//use core::alloc::
use crate::config::{MAX_BUFFER_SIZE, MAX_TASKS, SEMAPHORE_COUNT};
use crate::errors::KernelError;
use crate::kernel::semaphores::{SemaphoreControlBlock, Semaphores};
use crate::process::{get_pid, release};

use cortex_m_semihosting::hprintln;

pub type StaticBuffer = &'static [u32];

use crate::kernel::types::MessageId;

#[derive(Clone, Copy)]
pub struct TCB {
    dest_buffer: [u32; MAX_BUFFER_SIZE],
    msg_size: usize,
}

#[derive(Clone, Copy)]
pub struct MCB {
    pub receivers: u32,
    pub src_buffer: StaticBuffer,
}

#[derive(Clone, Copy)]
pub struct MessagingManager {
    pub tcb_table: [TCB; MAX_TASKS],
    pub mcb_table: [MCB; SEMAPHORE_COUNT],
    pub msg_scb_table: Semaphores,
}

impl TCB {
    pub const fn new() -> Self {
        Self {
            dest_buffer: [0; MAX_TASKS],
            msg_size: 0,
        }
    }
}

impl<'a> MessagingManager {
    pub fn broadcast(&mut self, msg_id: MessageId) -> Result<(), KernelError> {
        if self.mcb_table.get(msg_id).is_none() {
            return Err(KernelError::NotFound);
        }
        let mcb = self.mcb_table[msg_id];
        self.msg_scb_table
            .signal_and_release(msg_id, &mcb.receivers)?;
        return Ok(());
    }

    fn copy_msg(&mut self, msg_id: MessageId) {
        let src_msg = self.mcb_table[msg_id].src_buffer;
        let tasks_mask = self.mcb_table[msg_id].receivers;
        for tid in 1..MAX_TASKS {
            let tid_mask = (1 << tid) as u32;
            if tasks_mask & tid_mask == tid_mask {
                for i in 0..src_msg.len() {
                    self.tcb_table[tid].dest_buffer[i] = src_msg[i];
                }
                self.tcb_table[tid].msg_size = src_msg.len();
            }
        }
    }

    pub fn receive(&'a mut self, msg_id: MessageId) -> Option<&'a [u32]> {
        let rt = get_pid();
        self.copy_msg(msg_id);
        let tcb = &self.tcb_table[rt];
        match self.msg_scb_table.test_and_reset(msg_id) {
            Ok(res) if res == true => return Some(&tcb.dest_buffer[0..tcb.msg_size]),
            _ => return None,
        }
    }

    pub fn create(
        &mut self,
        tasks_mask: u32,
        receivers_mask: u32,
        src_buffer: StaticBuffer,
    ) -> Result<MessageId, KernelError> {
        if MAX_BUFFER_SIZE < src_buffer.len() {
            return Err(KernelError::BufferOverflow);
        }
        let msg_id = self.msg_scb_table.create(tasks_mask)?;
        self.mcb_table[msg_id].src_buffer = src_buffer;
        self.mcb_table[msg_id].receivers |= receivers_mask;
        return Ok(msg_id);
    }
}
