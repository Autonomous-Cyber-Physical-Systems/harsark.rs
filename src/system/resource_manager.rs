//! # Resource Manager
//! The Definition of Data-structures required for resource management.
//! The Resource manager handles the details of which processes have access to the which resource
//! and implements the locking and unlocking mechanism.

use crate::config::MAX_RESOURCES;
use crate::utils::arch::get_msb;
use crate::KernelError;
use crate::system::types::{BooleanVector, ResourceId, TaskId};

const PI: i32 = -1;

/// Describes a single Resource
#[derive(Clone, Copy)]
pub struct ResourceControlBlock {
    /// An boolean vector holding which tasks have access to the resource.
    ceiling: TaskId,
    /// It holds the priority of the highest priority task that can access that resource.
    tasks_mask: BooleanVector,
}

/// Manages Resources
#[derive(Clone, Copy)]
pub struct ResourceManager {
    /// An Array of ResourceControlBlock for every resource.
    resource_control_blocks: [Option<ResourceControlBlock>; MAX_RESOURCES],
    /// Points the top of the `pi_stack`.
    top: usize,
    /// This stack is used for locking and unlocking of resources.
    pi_stack: [i32; MAX_RESOURCES],
    /// It next empty index in the array where the new `ResourceControlBlock` can be stored.
    curr: usize,
    /// Hold the ceiling of the resource with the highest ceiling amongst the currently locked resources.
    system_ceiling: i32,
}

impl ResourceControlBlock {
    /// Instantiates a new ResourceControlBlock and returns it.
    pub fn new(tasks_mask: BooleanVector) -> Self {
        Self {
            ceiling: get_msb(tasks_mask) as TaskId,
            tasks_mask: tasks_mask,
        }
    }
}

impl ResourceManager {
    /// Returns a new instance of ResourceManager.
    pub const fn new() -> Self {
        ResourceManager {
            resource_control_blocks: [None; MAX_RESOURCES],
            top: 1,
            pi_stack: [PI; MAX_RESOURCES],
            curr: 0,
            system_ceiling: PI,
        }
    }

    /// Creates a new resource with the `task_mask` passed as argument. The new resource created is then pushed into `resource_control_blocks`.
    pub fn create(&mut self, tasks_mask: BooleanVector) -> Result<ResourceId, KernelError> {
        let id = self.curr;
        if id >= MAX_RESOURCES {
            return Err(KernelError::LimitExceeded);
        }
        self.resource_control_blocks[id].replace(ResourceControlBlock::new(tasks_mask));
        self.curr += 1;
        Ok(id)
    }

    /// This procedure locks the resource from further use. It is freed when unlock is called.
    /// By locking a resource, this function first checks if the task with TaskId `curr_tid` has access to
    /// the resource. If not, then return None. But If it has access, then generate the `task_mask`
    /// corresponding to the tasks that have to be blocked and return it.
    ///
    /// Note that though the resource is locked, it only blocks the tasks below its ceiling priority and
    /// not the others. Hence If any task with higher priority is released, it can lock resources with a
    /// higher ceiling. But Since only a task with higher priority can execute, it won’t allow the
    /// scheduler to execute tasks that would cause a cyclic wait for resources hence avoiding deadlocks.
    pub fn lock(&mut self, id: ResourceId, curr_tid: TaskId) -> Option<u32> {
        let resource = self.resource_control_blocks[id].unwrap();
        let ceiling = resource.ceiling;

        let pid_mask = 1 << curr_tid;

        if resource.tasks_mask & pid_mask != pid_mask {
            return None;
        }
        if ceiling as i32 > self.system_ceiling {
            self.push_stack(ceiling);

            let mask = self.get_pi_mask(ceiling) & !(1 << curr_tid);
            return Some(mask);
        }
        return None;
    }

    /// Returns the `Pi_mask`, which is just a boolean vector with all bits up to ceiling (including) set to 1.
    fn get_pi_mask(&self, ceiling: TaskId) -> u32 {
        let mask;
        if ceiling < 32 {
            mask = (1 << (ceiling + 1)) - 1;
        } else {
            mask = 0xffffffff
        }
        mask
    }

    /// This procedure would unlock the resource for other tasks if it were the lastly locked resource.
    /// It calls `pop_stack`, which pops the top of the stack and also re-assigns the `system_ceiling`.
    /// It returns a BooleanVector corresponding to the tasks that have to be unblocked.
    pub fn unlock(&mut self, id: ResourceId) -> Option<u32> {
        let resource = self.resource_control_blocks[id].unwrap();
        if resource.ceiling as i32 == self.system_ceiling {
            self.pop_stack();
            let mask = self.get_pi_mask(resource.ceiling);
            return Some(mask);
        }
        return None;
    }

    /// Pops the stack top and assigns the `system_ceiling` to the new stack top.
    fn pop_stack(&mut self) {
        self.top -= 1;
        self.system_ceiling = self.pi_stack[self.top - 1];
    }

    /// Pushes the passed ceiling onto the pi_stack.
    fn push_stack(&mut self, ceiling: TaskId) {
        self.pi_stack[self.top] = ceiling as i32;
        self.system_ceiling = ceiling as i32;
        self.top += 1;
    }
}
