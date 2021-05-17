//! # Resource Management Module
//!
//! Defines the Kernel routines and primitives for resource management.
use core::cell::RefCell;

use cortex_m::asm::bkpt;

use crate::kernel::tasks::{get_curr_tid, schedule, TaskManager};
use crate::system::pi_stack::PiStack;
use crate::system::scheduler::{BooleanVector, Context, TaskId};
use crate::utils::arch::{critical_section, Mutex};
use crate::utils::helpers::get_msb_const;
use crate::KernelError;

#[cfg(feature = "system_logger")]
use {crate::kernel::logging, crate::system::system_logger::LogEventType};

/// Global instance of Resource manager
static PiStackGlobal: Mutex<RefCell<PiStack>> = Mutex::new(RefCell::new(PiStack::new()));

/// A Safe Container to store a resource, it can hold resource of any Generic Type
/// and allow safe access to it without ending up in Data races or Deadlocks.
#[derive(Debug)]
pub struct Resource<T: Sized> {
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
    fn lock(&self, cxt: &Context) -> Result<&T, KernelError> {
        critical_section(|cs_token| {
            let pi_stack = &mut PiStackGlobal.borrow(cs_token).borrow_mut();
            let curr_tid = cxt.get_tid() as u32;

            let ceiling = self.ceiling;
            let pid_mask = 1 << curr_tid;
            if self.tasks_mask & pid_mask != pid_mask {
                return Err(KernelError::AccessDenied);
            }
            if ceiling as i32 > pi_stack.system_ceiling {
                pi_stack.push_stack(ceiling)?;
                let mask = Self::get_pi_mask(ceiling) & !(1 << curr_tid);
                TaskManager.borrow(cs_token).borrow_mut().block_tasks(mask);
                #[cfg(feature = "system_logger")]
                {
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
    fn unlock(&self) -> Result<(), KernelError> {
        let is_preemptive = critical_section(|cs_token| {
            let pi_stack = &mut PiStackGlobal.borrow(cs_token).borrow_mut();
            if self.ceiling as i32 == pi_stack.system_ceiling {
                pi_stack.pop_stack()?;
                let mask = Self::get_pi_mask(self.ceiling);
                TaskManager
                    .borrow(cs_token)
                    .borrow_mut()
                    .unblock_tasks(mask);
            }
            #[cfg(feature = "system_logger")]
            {
                if logging::get_resource_unlock() {
                    logging::report(LogEventType::ResourceUnlock(get_curr_tid() as u32));
                }
            }
            Ok(TaskManager.borrow(cs_token).borrow_mut().is_preemptive)
        })?;
        schedule(is_preemptive);
        Ok(())
    }
    /// A helper function that ensures that if a resource is locked, it is unlocked.
    pub fn acquire<F, R>(&self, cxt: &Context, mut handler: F) -> Result<R, KernelError>
    where
        F: FnMut(&T) -> R,
    {
        // Context with priority zero is called during kernel initialization, so there is no locking happening here as such.
        if cxt.get_tid() == 0 {
            return Ok(handler(&self.inner));
        }
        let value = self.lock(cxt)?;
        let res = handler(value);
        self.unlock()?;
        return Ok(res);
    }
}

unsafe impl<T> Sync for Resource<T> {}
