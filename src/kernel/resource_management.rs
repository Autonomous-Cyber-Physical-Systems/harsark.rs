use crate::config::MAX_TASKS;
use crate::event_manager::execute_event;
use crate::task_manager::{block_tasks, preempt, unblock_tasks};
use core::cmp::max;
use core::pin::Pin;
use cortex_m_semihosting::hprintln;
use crate::helper::get_msb;
pub type ResourceId = usize;

const PI: u32 = 0;

pub struct ResourceManager {
    RCB: [u32; MAX_TASKS], // Resource Control Block, holds u32 expressing which tasks have access to it.
    top: usize,
    pi_stack: [u32; MAX_TASKS],
    curr: usize, // used to track current no. of resources initialized
    system_ceiling: u32
}

impl ResourceManager {
    pub fn init () -> Self {
        ResourceManager {
            RCB: [PI; MAX_TASKS],
            top: 0,
            pi_stack: [0; MAX_TASKS],
            curr: 0,
            system_ceiling: PI
        }
    }

    pub fn create(&mut self ,tasks_mask: u32)  -> ResourceId {
        let id = self.curr;
        self.RCB[id] = get_msb(&tasks_mask) as u32;
        self.curr += 1;
        return id;
    }

    pub fn lock(&mut self, id: ResourceId) {
        let rt_ceiling = self.RCB[id];
        if rt_ceiling > self.system_ceiling {
            self.push_stack(rt_ceiling);

            // can be optimized
            let mut mask = 0;
            for i in 0..(rt_ceiling - 1) {
                mask |= 1 << i;
            }

            self.system_ceiling = self.RCB[id];
            block_tasks(mask);
        }
    }

    pub fn unlock(&mut self, id: ResourceId) {
        let rt_ceiling = self.RCB[id];
        if rt_ceiling == self.system_ceiling {
            self.pop_stack();
            let mut mask = 0;
            for i in 0..(rt_ceiling - 1) {
                mask &= 1 << i;
            }
            unblock_tasks(mask);
            preempt();
        }
    }

    fn pop_stack(&mut self) {
        self.system_ceiling = self.pi_stack[self.top - 1];
        self.top -= 1;
    }

    fn push_stack(&mut self, ceiling: u32) {
        self.pi_stack[self.top] = self.system_ceiling;
        self.top += 1;
    }
}
