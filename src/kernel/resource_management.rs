//! # Resource Management Module
//! Defines the Kernel routines and primitives for resource management.
//!
use core::cell::RefCell;

use cortex_m::interrupt::free as execute_critical;
use cortex_m::interrupt::Mutex;

use crate::priv_execute;
use crate::system::resource_manager::ResourceManager;
use crate::system::types::{BooleanVector, ResourceId};
use crate::utils::arch::is_privileged;
use crate::KernelError;
use crate::kernel::task_management::{block_tasks, get_curr_tid, schedule, unblock_tasks};

/// Global instance of Resource manager
static resources_list: Mutex<RefCell<ResourceManager>> =
    Mutex::new(RefCell::new(ResourceManager::new()));

/// A Safe Container to store a resource, it can hold resource of any Generic Type
/// and allow safe access to it without leading into Data races or Deadlocks.
#[derive(Debug)]
pub struct Resource<T: Sized> {
    /// This field holds the actual resource that has to be locked.
    inner: T,
    /// Holds the ResourceId allotted by the resource manager for the resource.
    id: ResourceId,
}

impl<T> Resource<T> {
    /// Returns a new instance of Resource created from the arguments
    pub fn new(val: T, id: ResourceId) -> Self {
        Self { inner: val, id }
    }

    /// Lock the resources. It takes a BooleanVector corresponding to the tasks that have to be blocked
    /// from `resources_list.lock()` and calls `block_tasks()` on it.
    fn lock(&self) -> Option<&T> {
        execute_critical(|cs_token| {
            let pid = if is_privileged() {
                0
            } else {
                get_curr_tid() as u32
            };
            let res = resources_list
                .borrow(cs_token)
                .borrow_mut()
                .lock(self.id, pid);
            if let Some(mask) = res {
                block_tasks(mask);
                return Some(&self.inner);
            }
            return None;
        })
    }

    /// Unlocks the resource. It takes a BooleanVector corresponding to the tasks that have to be
    /// unblocked from `resource_manager.unlock()` and calls `unblock_tasks()` on it.
    fn unlock(&self) {
        execute_critical(|cs_token| {
            if let Some(mask) = resources_list.borrow(cs_token).borrow_mut().unlock(self.id) {
                unblock_tasks(mask);
                schedule();
            }
        })
    }

    /// Acquire is a helper function that ensures that if a resource is locked, it is unlocked too.
    /// It takes one argument handler, which is function closure that takes a parameter of type `&T`.
    /// If the resource is free, the the handler is executed with `inner` as the parameter.
    pub fn acquire<F>(&self, handler: F)
    where
        F: Fn(&T),
    {
        if let Some(res) = self.lock() {
            handler(res);
            self.unlock();
        }
    }

    /// There might be cases where the variable has to be accessed without locks for some reason.
    /// This function is used to access the resource bypassing the locking system,
    /// and it returns a reference to `self.inner`. This function is explicitly marked unsafe.
    pub unsafe fn access(&self) -> Result<&T, KernelError> {
        Ok(&self.inner)
    }
}

/// It is used to instantiate a new Resource. This function takes ownership of the variable.
/// It returns a resource instantiated with the value. Hence ensuring the value cannot be accessed
/// without calls to `acquire` or `access`.
pub fn new<T: Sized>(resource: T, tasks_mask: BooleanVector) -> Result<Resource<T>, KernelError> {
    // External interrupts and Privileged tasks have a priority of 0
    let tasks_mask = tasks_mask | 1 << 0;
    priv_execute!({
        execute_critical(|cs_token| {
            let id = resources_list
                .borrow(cs_token)
                .borrow_mut()
                .create(tasks_mask)?;
            Ok(Resource::new(resource, id))
        })
    })
}

/// This function instantiates the `cortex_m::Peripherals` struct, wraps it in a resource container,
/// and returns it. This peripheral instance is instrumental in configuring the GPIO pins on the board,
/// clock, etc.
pub fn init_peripherals() -> Result<Resource<RefCell<cortex_m::Peripherals>>, KernelError> {
    let mask: u32 = 0xffffffff;
    new(RefCell::new(cortex_m::Peripherals::take().unwrap()), mask)
}

unsafe impl<T> Sync for Resource<T> {}
