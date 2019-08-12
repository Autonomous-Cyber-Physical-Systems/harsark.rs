use crate::task_manager::{
    release,
    get_RT
};
use cortex_m::interrupt::free as execute_critical;

#[derive(Clone, Copy)]
pub struct SCB {
    pub flags: [bool; 32],
    pub tasks: [bool; 32],
}

static mut SCB_TABLE: [SCB; 32] = [SCB {
    flags: [false; 32],
    tasks: [false; 32],
}; 32];

pub fn signal_and_release(semaphore: usize, tasks: &[usize]) {
    execute_critical(|_| {
        let scb_table = unsafe { &mut SCB_TABLE };
        for tid in tasks {
            scb_table[semaphore].flags[*tid] = true;
        }
        release(&scb_table[semaphore].tasks);
    })
}

pub fn semaphore_set_tasks(semaphore: usize, tasks: &[usize]) {
    execute_critical(|_| {
        let scb_table = unsafe { &mut SCB_TABLE };
        for tid in tasks {
            scb_table[semaphore].tasks[*tid] = true;
        }
    });
}

pub fn test_and_reset(semaphore: usize) -> bool {
    execute_critical(|_| {
        let scb_table = unsafe { &mut SCB_TABLE };
        let rt = get_RT();
        if scb_table[semaphore].flags[rt] == true {
            scb_table[semaphore].flags[rt] = false;
            return true;
        } else {
            return false;
        }
    })
}
