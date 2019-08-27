use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::interrupt::free as execute_critical;

use crate::kernel::resource_management::{ResourceManager, ResourceId};
use crate::helper::generate_task_mask;
use crate::errors::KernelError;

lazy_static!{
    static ref Resources: Mutex<RefCell<ResourceManager>> = Mutex::new(RefCell::new(ResourceManager::init()));
}

pub fn new(tasks: &[u32])  -> Result<ResourceId,KernelError> {
    execute_critical(|cs_token| {
        Resources
            .borrow(cs_token)
            .borrow_mut()
            .new(&generate_task_mask(tasks))
    })
}

pub fn lock(id: ResourceId) {
    execute_critical(|cs_token| {
        Resources
            .borrow(cs_token)
            .borrow_mut()
            .lock(id)
    })
}

pub fn unlock(id: ResourceId) {
    execute_critical(|cs_token| {
        Resources
            .borrow(cs_token)
            .borrow_mut()
            .unlock(id)
    })
}