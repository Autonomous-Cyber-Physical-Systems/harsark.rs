
use crate::priv_execute;
use crate::system::resource_manager::ResourceManager;
use crate::system::types::ResourceId;
use crate::utils::arch::is_privileged;
use crate::KernelError;
use core::cell::RefCell;
use cortex_m::interrupt::free as execute_critical;
use cortex_m::interrupt::Mutex;

use crate::kernel::tasks::{block_tasks, get_curr_tid, schedule, unblock_tasks};



static resources_list: Mutex<RefCell<ResourceManager>> =
    Mutex::new(RefCell::new(ResourceManager::new()));

#[derive(Debug)]
pub struct Resource<T: Sized> {
    inner: T,
    id: ResourceId,
}

impl<T> Resource<T> {
    pub fn new(val: T, id: ResourceId) -> Self {
        Self { inner: val, id }
    }

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

    fn unlock(&self) {
        execute_critical(|cs_token| {
            if let Some(mask) = resources_list.borrow(cs_token).borrow_mut().unlock(self.id) {
                unblock_tasks(mask);
                schedule();
            }
        })
    }

    pub fn acquire<F>(&self, handler: F)
    where
        F: Fn(&T),
    {
        if let Some(res) = self.lock() {
            handler(res);
            self.unlock();
        }
    }

    pub unsafe fn access(&self) -> Result<&T, KernelError> {
        Ok(&self.inner)
    }
}

pub fn new<T: Sized>(resource: T, tasks_mask: u32) -> Result<Resource<T>, KernelError> {
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

pub fn init_peripherals() -> Result<Resource<RefCell<cortex_m::Peripherals>>, KernelError> {
    let mask: u32 = 0xffffffff;
    new(RefCell::new(cortex_m::Peripherals::take().unwrap()), mask)
}

unsafe impl<T> Sync for Resource<T> {}
