//! # Resource Manager
//! The Definition of Data-structures required for resource management.
//! The Resource manager handles the details of which processes have access to the which resource
//! and implements the locking and unlocking mechanism.

use crate::config::MAX_RESOURCES;
use crate::utils::arch::get_msb;
use crate::KernelError;
use crate::system::types::{BooleanVector, ResourceId, TaskId};

const PI: i32 = -1;

pub struct PiStack {
    /// Points the top of the `pi_stack`.
    top: usize,
    /// This stack is used for locking and unlocking of resources.
    pi_stack: [i32; MAX_RESOURCES],
    /// Hold the ceiling of the resource with the highest ceiling amongst the currently locked resources.
    pub system_ceiling: i32,
}

impl PiStack {
    pub const fn new() -> Self {
        Self {
            top: 1,
            pi_stack: [PI; MAX_RESOURCES],
            system_ceiling: PI,
        }
    }

    /// Pops the stack top and assigns the `system_ceiling` to the new stack top.
    pub fn pop_stack(&mut self) {
        self.top -= 1;
        self.system_ceiling = self.pi_stack[self.top - 1];
    }

    /// Pushes the passed ceiling onto the pi_stack.
    pub fn push_stack(&mut self, ceiling: TaskId) -> Result<(),KernelError> {
        if self.top >= MAX_RESOURCES {
            return Err(KernelError::MaxResources)
        }
        self.pi_stack[self.top] = ceiling as i32;
        self.system_ceiling = ceiling as i32;
        self.top += 1;
        Ok(())
    }
}