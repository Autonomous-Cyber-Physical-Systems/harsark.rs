//! # Resource Management Module
//!
//! Defines the Kernel routines and primitives for resource management.
use core::cell::{RefCell};

use crate::utils::arch::{Mutex, critical_section};
use crate::utils::helpers::get_msb_const;
use crate::system::pi_stack::PiStack;
use crate::KernelError;
use crate::kernel::tasks::{block_tasks, get_curr_tid, schedule, unblock_tasks};
use crate::system::scheduler::{TaskId, BooleanVector};

#[cfg(feature = "system_logger")]
use {
    crate::system::system_logger::LogEventType,
    crate::kernel::logging,
};

/// Global instance of Resource manager
static PiStackGlobal: Mutex<RefCell<PiStack>> = Mutex::new(RefCell::new(PiStack::new()));

/// A Safe Container to store a resource, it can hold resource of any Generic Type
/// and allow safe access to it without ending up in Data races or Deadlocks.
#[derive(Debug)]
pub struct Resource<T: Sized> 
{
    /// An boolean vector holding which tasks have access to the resource.
    ceiling: TaskId,
    /// It holds the priority of the highest priority task that can access that resource.
    tasks_mask: BooleanVector,
    /// This field holds the actual resource that has to be locked.
    inner: T,
}

impl<T: Sized> Resource<T> {
    
    /// Create and initialize new Resource object
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
    
    /// Lock the Resource for the currently running task and blocks the competing tasks 
    fn lock(&self) -> Result<&T,KernelError> {
        critical_section(|cs_token| {
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
                #[cfg(feature = "system_logger")] {
                    if logging::get_resource_lock() {
                        logging::report(LogEventType::ResourceLock(curr_tid));
                    }
                }
                return Ok(&self.inner);
            }
            return Err(KernelError::AccessDenied);
        })
    }

    /// Unlocks the Resource and unblocks the tasks which were blocked during the call to lock
    fn unlock(&self) -> Result<(),KernelError> {
        critical_section(|cs_token| {
            let pi_stack = &mut PiStackGlobal.borrow(cs_token).borrow_mut();
            if self.ceiling as i32 == pi_stack.system_ceiling {
                pi_stack.pop_stack()?;
                let mask = Self::get_pi_mask(self.ceiling);
                unblock_tasks(mask);
                schedule();
            }
            #[cfg(feature = "system_logger")] {
                if logging::get_resource_unlock() {
                    logging::report(LogEventType::ResourceUnlock(get_curr_tid() as u32));
                }
            }
            Ok(())
        })
    }
    /// A helper function that ensures that if a resource is locked, it is unlocked.
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

unsafe impl<T> Sync for Resource<T> {}