use crate::config::MAX_RESOURCES;
use crate::KernelError;
use crate::utils::arch::get_msb;

use crate::system::types::ResourceId;
use cortex_m_semihosting::hprintln;

const PI: i32 = -1;

#[derive(Clone, Copy)]
pub struct ResourceControlBlock {
    ceiling: u32,
    tasks_mask: u32,
}

#[derive(Clone, Copy)]
pub struct ResourceManager {
    resource_control_blocks: [Option<ResourceControlBlock>; MAX_RESOURCES], // Resource Control Block, holds u32 expressing which tasks have access to it.
    top: usize,
    pi_stack: [i32; MAX_RESOURCES],
    curr: usize, // used to track current no. of resources initialized
    system_ceiling: i32,
}

impl ResourceControlBlock {
    pub fn new(tasks_mask: u32) -> Self {
        Self {
            ceiling: get_msb(tasks_mask) as u32,
            tasks_mask: tasks_mask,
        }
    }
}

impl ResourceManager {
    pub const fn new() -> Self {
        ResourceManager {
            resource_control_blocks: [None; MAX_RESOURCES],
            top: 1,
            pi_stack: [PI; MAX_RESOURCES],
            curr: 0,
            system_ceiling: PI,
        }
    }

    pub fn create(&mut self, tasks_mask: u32) -> Result<ResourceId, KernelError> {
        let id = self.curr;
        if id >= MAX_RESOURCES {
            return Err(KernelError::LimitExceeded);
        }
        self.resource_control_blocks[id].replace(ResourceControlBlock::new(tasks_mask));
        self.curr += 1;
        Ok(id)
    }

    pub fn lock(&mut self, id: ResourceId, curr_pid: u32) -> Option<u32> {
        let resource = self.resource_control_blocks[id].unwrap();
        let ceiling = resource.ceiling;

        let pid_mask = 1 << curr_pid;

        if resource.tasks_mask & pid_mask != pid_mask {
            return None;
        }
        if ceiling as i32 > self.system_ceiling {
            self.push_stack(ceiling);

            let mask = self.get_pi_mask(ceiling)  & !(1 << curr_pid);
            return Some(mask);
        }
        return None;
    }

    fn get_pi_mask(&self, ceiling: u32) -> u32 {
        let mut mask = 0;
        if ceiling < 32 {
            mask = (1 << (ceiling + 1)) - 1;
        } else {
            mask = 0xffffffff
        }
        mask
    }

    pub fn unlock(&mut self, id: ResourceId) -> Option<u32> {
        let resource = self.resource_control_blocks[id].unwrap();
        if resource.ceiling as i32 == self.system_ceiling {
            self.pop_stack();
            let mask = self.get_pi_mask(resource.ceiling);
            return Some(mask);
        }
        return None;
    }

    fn pop_stack(&mut self) {
        self.top -= 1;
        self.system_ceiling = self.pi_stack[self.top - 1];
    }

    fn push_stack(&mut self, ceiling: u32) {
        self.pi_stack[self.top] = ceiling as i32;
        self.system_ceiling = ceiling as i32;
        self.top += 1;
    }
}
