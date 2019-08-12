use crate::task_manager::release;
use cortex_m::interrupt::free as execute_critical;
#[derive(Clone, Copy)]
struct SCB {
    flags: [bool; 32],
    tasks: [bool; 32],
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
