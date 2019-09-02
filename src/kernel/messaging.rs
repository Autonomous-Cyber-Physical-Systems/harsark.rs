#![feature(const_fn)]
//use core::alloc::
use crate::config::{MAX_BUFFER_SIZE, MAX_TASKS, SEMAPHORE_COUNT};
use crate::errors::KernelError;
use crate::kernel::semaphores::{SemaphoreControlBlock, SemaphoreId, Semaphores};
use crate::kernel::task_manager::{get_RT, release};

use crate::helper::generate_task_mask;
use cortex_m_semihosting::hprintln;

pub type StaticBuffer = &'static [u32];

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
    pub fn broadcast(&mut self, sem_id: SemaphoreId) -> Result<(), KernelError> {
        if self.mcb_table.get(sem_id).is_none() {
            return Err(KernelError::NotFound);
        }
        let mcb = self.mcb_table[sem_id];
        self.copy_msg(sem_id)?;
        self.msg_scb_table
            .signal_and_release(sem_id, &mcb.receivers)?;
        return Ok(());
    }

    fn copy_msg(&mut self, sem_id: SemaphoreId) -> Result<(), KernelError> {
        let src_msg = self.mcb_table[sem_id].src_buffer;
        let tasks_mask = self.mcb_table[sem_id].receivers;
        if MAX_BUFFER_SIZE < src_msg.len() {
            return Err(KernelError::BufferOverflow);
        }
        for tid in 1..MAX_TASKS {
            let tid_mask = (1 << tid) as u32;
            if tasks_mask & tid_mask == tid_mask {
                for i in 0..src_msg.len() {
                    self.tcb_table[tid].dest_buffer[i] = src_msg[i];
                }
                self.tcb_table[tid].msg_size = src_msg.len();
            }
        }
        return Ok(());
    }

    pub fn receive(&'a mut self, sem_id: SemaphoreId) -> Option<&'a [u32]> {
        let rt = get_RT();
        let tcb = &self.tcb_table[rt];
        match self.msg_scb_table.test_and_reset(sem_id) {
            Ok(res) if res == true => return Some(&tcb.dest_buffer[0..tcb.msg_size]),
            _ => return None,
        }
        return None;
    }

    pub fn create(
        &mut self,
        tasks: &[u32],
        receivers: &[u32],
        src_buffer: StaticBuffer,
    ) -> Result<SemaphoreId, KernelError> {
        let sem_id = self.msg_scb_table.create(generate_task_mask(tasks))?;
        self.mcb_table[sem_id].src_buffer = src_buffer;
        self.mcb_table[sem_id].receivers |= generate_task_mask(receivers);
        return Ok(sem_id);
    }
}
