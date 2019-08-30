//use core::alloc::
use crate::config::{MAX_BUFFER_SIZE, MAX_TASKS, SEMAPHORE_COUNT};
use crate::errors::KernelError;
use crate::kernel::semaphores::{Semaphores, SemaphoreControlBlock, SemaphoreId};
use crate::kernel::task_manager::{get_RT, release};

use cortex_m::interrupt::free as execute_critical;
use cortex_m_semihosting::hprintln;
use crate::helper::generate_task_mask;

pub type Buffer = &'static [u32];

#[derive(Clone, Copy)]
struct TCB {
    dest_buffer: [u32; MAX_BUFFER_SIZE],
    msg_size: usize,
}

#[derive(Clone, Copy)]
struct MCB {
    receivers: u32,
    src_buffer: Buffer,
}

struct MessagingManager {
    tcb_table: [TCB; MAX_TASKS],
    mcb_table: [MCB; SEMAPHORE_COUNT],
    msg_scb_table: Semaphores
}

impl<'a> MessagingManager {
    pub fn new () -> Self{
        Self {
            tcb_table: [TCB {
                dest_buffer: [0; MAX_TASKS],
                msg_size: 0,
            }; MAX_TASKS],
            mcb_table: [MCB {
                receivers: 0,
                src_buffer: &[],
            }; SEMAPHORE_COUNT],
            msg_scb_table: Semaphores {
                table: [SemaphoreControlBlock { flags: 0, tasks: 0 }; SEMAPHORE_COUNT],
                curr: 0
            }
        }
    }

    pub fn broadcast(&mut self, var: SemaphoreId) -> Result<(), KernelError> {
        execute_critical(|_| {
            if self.mcb_table.get(var).is_none() {
                return Err(KernelError::NotFound);
            }
            let mcb = self.mcb_table[var];
            self.copy_msg(var)?;
            self.msg_scb_table.signal_and_release(var, &mcb.receivers)?;
            return Ok(());
        })
    }

    fn copy_msg(&mut self, var: SemaphoreId) -> Result<(), KernelError> {
        let src_msg = self.mcb_table[var].src_buffer;
        let tasks_mask = self.mcb_table[var].receivers;
        if MAX_BUFFER_SIZE < src_msg.len() {
            return Err(KernelError::BufferOverflow);
        }
        for tid in 1..MAX_TASKS{
            let tid_mask = (1 << tid) as u32 ;
            if tasks_mask & tid_mask == tid_mask {
                for i in 0..src_msg.len() {
                    self.tcb_table[tid].dest_buffer[i] = src_msg[i];
                }
                self.tcb_table[tid].msg_size = src_msg.len();
            }
        }
        return Ok(());
    }

    pub fn receive(&'a mut self, var: SemaphoreId) -> Option<&'a [u32]> {
        execute_critical(move |_| {
            let rt = get_RT();
            let tcb = &self.tcb_table[rt];
            match self.msg_scb_table.test_and_reset(var) {
                Ok(res) if res == true => {
                    return Some(&tcb.dest_buffer[0..tcb.msg_size])
                }
                _ => return None,
            }
            return None
        })
    }

    pub fn create(&mut self, tasks: &[u32], receivers: &[u32], src_buffer: Buffer) -> Result<SemaphoreId,KernelError> {
        execute_critical(|_| {
            let sem_id = self.msg_scb_table.new(generate_task_mask(tasks))?;
            self.mcb_table[sem_id].src_buffer = src_buffer;
            self.mcb_table[sem_id].receivers |= generate_task_mask(receivers);
            return Ok(sem_id);
        })
    }
}
