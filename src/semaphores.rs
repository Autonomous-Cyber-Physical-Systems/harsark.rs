use crate::config::SEMAPHORE_COUNT;
use crate::errors::KernelError;
use crate::task_manager::{get_RT, release};
use cortex_m::interrupt::free as execute_critical;

#[derive(Clone, Copy)]
pub struct SCB {
    pub flags: u32,
    pub tasks: u32,
}

static mut SCB_TABLE: [SCB; 32] = [SCB { flags: 0, tasks: 0 }; SEMAPHORE_COUNT];

pub fn signal_and_release(semaphore: usize, tasks_mask: &u32) -> Result<(), KernelError> {
    execute_critical(|_| {
        let scb_table = unsafe { &mut SCB_TABLE };
        if scb_table.get(semaphore).is_none() {
            return Err(KernelError::NotFound);
        }
        scb_table[semaphore].flags |= *tasks_mask;
        release(&scb_table[semaphore].tasks);
        return Ok(());
    })
}

pub fn semaphore_set_tasks(semaphore: usize, tasks_mask: &u32) -> Result<(), KernelError> {
    execute_critical(|_| {
        let scb_table = unsafe { &mut SCB_TABLE };
        if scb_table.get(semaphore).is_none() {
            return Err(KernelError::NotFound);
        }
        scb_table[semaphore].tasks |= *tasks_mask;
        Ok(())
    })
}

pub fn test_and_reset(semaphore: usize) -> Result<bool, KernelError> {
    execute_critical(|_| {
        let scb_table = unsafe { &mut SCB_TABLE };
        let rt = get_RT() as u32;
        if scb_table.get(semaphore).is_none() {
            return Err(KernelError::NotFound);
        }
        let rt_mask = (1 << rt);
        if scb_table[semaphore].flags & rt_mask == rt_mask {
            scb_table[semaphore].flags &= !rt_mask;
            return Ok(true);
        } else {
            return Ok(false);
        }
    })
}
