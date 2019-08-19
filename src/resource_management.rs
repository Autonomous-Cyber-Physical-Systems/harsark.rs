use crate::config::MAX_TASKS;
use crate::event_manager::execute_event;
use crate::task_manager::{block_tasks, preempt, unblock_tasks};
use core::cmp::max;
use core::pin::Pin;
use cortex_m_semihosting::hprintln;

const PI: u32 = 0;
static mut RCB: [u32; MAX_TASKS] = [PI; MAX_TASKS];

static mut TOP: usize = 0;
static mut PI_STACK: [u32; MAX_TASKS] = [0; MAX_TASKS];
static mut SYSTEM_CEILING: u32 = PI;

pub fn set_permitted_tasks(id: usize, tasks_mask: u32) {
    for tid in (0..MAX_TASKS).rev() {
        unsafe {
            let mask = 1 << tid as u32;
            if tasks_mask & mask == mask {
                RCB[id] = max(RCB[id], mask);
            }
        }
    }
}

pub fn lock(id: usize) {
    let rt_ceiling = unsafe { RCB[id] };
    if rt_ceiling > unsafe { SYSTEM_CEILING } {
        push_stack(rt_ceiling);
        let mut mask = 0;
        for i in 0..(rt_ceiling - 1) {
            mask |= 1 << i;
        }
        unsafe {
            SYSTEM_CEILING = RCB[id];
            hprintln!("{}", mask);
            block_tasks(mask);
        }
    }
}

pub fn unlock(id: usize) {
    let rt_ceiling = unsafe { RCB[id] };
    if rt_ceiling == unsafe { SYSTEM_CEILING } {
        pop_stack();
        let mut mask = 0;
        for i in 0..(rt_ceiling - 1) {
            mask &= 1 << i;
        }
        unblock_tasks(mask);
        preempt();
    }
}

fn pop_stack() {
    unsafe {
        SYSTEM_CEILING = PI_STACK[TOP - 1];
        TOP -= 1;
    }
}

fn push_stack(ceiling: u32) {
    unsafe {
        PI_STACK[TOP] = SYSTEM_CEILING;
        TOP += 1;
    }
}
