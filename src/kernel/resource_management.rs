use crate::process::get_pid;
use crate::config::MAX_RESOURCES;
use crate::errors::KernelError;
use crate::kernel::helper::get_msb;
use crate::process::{block_tasks, schedule, unblock_tasks};
use core::cmp::max;
use core::pin::Pin;
use cortex_m_semihosting::hprintln;

use crate::kernel::types::ResourceId;

const PI: u32 = 0;

#[derive(Clone, Copy)]
pub struct ResourceManager {
    resources_block: [u32; MAX_RESOURCES], // Resource Control Block, holds u32 expressing which tasks have access to it.
    top: usize,
    pi_stack: [u32; MAX_RESOURCES],
    curr: usize, // used to track current no. of resources initialized
    system_ceiling: u32,
}

impl ResourceManager {
    pub const fn new() -> Self {
        ResourceManager {
            resources_block: [PI; MAX_RESOURCES],
            top: 0,
            pi_stack: [0; MAX_RESOURCES],
            curr: 0,
            system_ceiling: PI,
        }
    }

    pub fn create(&mut self, tasks_mask: &u32) -> Result<ResourceId, KernelError> {
        let id = self.curr;
        if id >= MAX_RESOURCES {
            return Err(KernelError::LimitExceeded);
        }
        self.resources_block[id] = get_msb(&tasks_mask) as u32;
        self.curr += 1;
        Ok(id)
    }

    pub fn lock(&mut self, id: ResourceId) -> bool {
        let rt_ceiling = self.resources_block[id];
        let curr_pid = get_pid();
        if rt_ceiling > self.system_ceiling {
            self.push_stack(rt_ceiling);

            // can be optimized
            let mut mask = 0;
            for i in 0..(rt_ceiling - 1) {
                mask |= 1 << i;
            }
            mask &= !(1<<curr_pid);
        
            self.system_ceiling = self.resources_block[id];
            block_tasks(mask);
            return true;
        }
        return false;
    }

    pub fn unlock(&mut self, id: ResourceId) {
        let rt_ceiling = self.resources_block[id];
        if rt_ceiling == self.system_ceiling {
            self.pop_stack();
            let mut mask = 0;
            for i in 0..(rt_ceiling - 1) {
                mask &= 1 << i;
            }
            unblock_tasks(mask);
            schedule();
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
