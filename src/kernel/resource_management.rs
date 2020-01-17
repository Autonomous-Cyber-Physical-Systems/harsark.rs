//! # Resource Management Module
//! Defines the Kernel routines and primitives for resource management.
//!
use core::cell::RefCell;

use cortex_m::interrupt::free as execute_critical;
use cortex_m::interrupt::Mutex;

use crate::priv_execute;
use crate::utils::arch::{get_msb_const,get_msb};
use crate::system::resource_manager::PiStack;
use crate::utils::arch::is_privileged;
use crate::KernelError;
use crate::kernel::task_management::{block_tasks, get_curr_tid, schedule, unblock_tasks};
use crate::system::types::{BooleanVector, TaskId};
use cortex_m_semihosting::hprintln;

/// Global instance of Resource manager
static PiStackGlobal: Mutex<RefCell<PiStack>> = Mutex::new(RefCell::new(PiStack::new()));

/// A Safe Container to store a resource, it can hold resource of any Generic Type
/// and allow safe access to it without leading into Data races or Deadlocks.
#[derive(Debug)]
pub struct Resource<T: Sized> {
    /// An boolean vector holding which tasks have access to the resource.
    ceiling: TaskId,
    /// It holds the priority of the highest priority task that can access that resource.
    tasks_mask: BooleanVector,
    /// This field holds the actual resource that has to be locked.
    inner: T,
}

impl<T> Resource<T> {
    
    /// It is used to instantiate a new Resource. This function takes ownership of the variable.
    /// It returns a resource instantiated with the value. Hence ensuring the value cannot be accessed
    /// without calls to `acquire` or `access`.
    pub const fn new(val: T, tasks_mask: BooleanVector) -> Self {
        let tasks_mask = tasks_mask | 1;
        Self { 
            inner: val,
            tasks_mask: tasks_mask,
            ceiling: get_msb_const(tasks_mask) as TaskId,
        }
    }

    /// Returns the `Pi_mask`, which is just a boolean vector with all bits up to ceiling (including) set to 1.
    fn get_pi_mask(ceiling: TaskId) -> u32 {
        let mask;
        if ceiling < 31 {
                mask = (1 << (ceiling + 1)) - 1;
        } else {
            mask = 0xffffffff
        }
        mask
    }
    
    /// Lock the resources. It takes a BooleanVector corresponding to the tasks that have to be blocked
    /// from `resources_list.lock()` and calls `block_tasks()` on it.
    fn lock(&self) -> Result<&T,KernelError> {
        execute_critical(|cs_token| {
            let pi_stack = &mut PiStackGlobal.borrow(cs_token).borrow_mut();
            let curr_tid = get_curr_tid() as u32;
            
            let ceiling = self.ceiling;
            let pid_mask = 1 << curr_tid;
            if self.tasks_mask & pid_mask != pid_mask {
                return Err(KernelError::AccessDenied);
            }
            if ceiling as i32 > pi_stack.system_ceiling {
                pi_stack.push_stack(ceiling)?;
                let mask = Self::get_pi_mask(ceiling) & !(1 << curr_tid);
                block_tasks(mask);
                return Ok(&self.inner);
            }
            return Err(KernelError::AccessDenied);
        })
    }

    /// Unlocks the resource. It takes a BooleanVector corresponding to the tasks that have to be
    /// unblocked from `resource_manager.unlock()` and calls `unblock_tasks()` on it.
    fn unlock(&self) -> Result<(),KernelError> {
        execute_critical(|cs_token| {
            let pi_stack = &mut PiStackGlobal.borrow(cs_token).borrow_mut();
            if self.ceiling as i32 == pi_stack.system_ceiling {
                pi_stack.pop_stack()?;
                let mask = Self::get_pi_mask(self.ceiling);
                unblock_tasks(mask);
                schedule();
            }
            Ok(())
        })
    }
    /// Acquire is a helper function that ensures that if a resource is locked, it is unlocked too.
    /// It takes one argument handler, which is function closure that takes a parameter of type `&T`.
    /// If the resource is free, the the handler is executed with `inner` as the parameter.
    pub fn acquire<F,R>(&self, handler: F) -> Result<R,KernelError>
    where
        F: Fn(&T) -> R,
    {
        let value = self.lock()?;
        let res = handler(value);
        self.unlock()?;
        return Ok(res);
    }
}

/// This function instantiates the `cortex_m::Peripherals` struct, wraps it in a resource container,
/// and returns it. This peripheral instance is instrumental in configuring the GPIO pins on the board,
/// clock, etc.
pub fn init_peripherals() -> Resource<RefCell<cortex_m::Peripherals>> {
    let mask: u32 = 0xffffffff;
    Resource::new(RefCell::new(cortex_m::Peripherals::take().unwrap()), mask)
}

unsafe impl<T> Sync for Resource<T> {}