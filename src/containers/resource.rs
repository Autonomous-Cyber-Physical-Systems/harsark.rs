use core::cell::{RefCell, RefMut};
use cortex_m::interrupt::free as execute_critical;
use cortex_m::interrupt::Mutex;

use crate::errors::KernelError;
use crate::kernel::helper::generate_task_mask;
use crate::kernel::resource_management::ResourceManager;

use crate::kernel::types::ResourceId;

static resources_list: Mutex<RefCell<ResourceManager>> =
    Mutex::new(RefCell::new(ResourceManager::new()));

pub struct Resource<T: Sized> {
    inner: RefCell<T>,
    id: ResourceId,
}

impl<T> Resource<T> {
    pub fn new(val: T, id: ResourceId) -> Self {
        Self {
            inner: RefCell::new(val),
            id,
        }
    }

    pub fn lock(&self) -> Option<&RefCell<T>> {
        execute_critical(|cs_token| {
            let res = resources_list.borrow(cs_token).borrow_mut().lock(self.id);
            if res {
                return Some(&self.inner);
            }
            return None;
        })
    }

    pub fn unlock(&self) {
        execute_critical(|cs_token| resources_list.borrow(cs_token).borrow_mut().unlock(self.id))
    }
}

pub fn create<T: Sized>(resource: T, tasks: &[u32]) -> Result<Resource<T>, KernelError> {
    execute_critical(|cs_token| {
        let id = resources_list
            .borrow(cs_token)
            .borrow_mut()
            .create(&generate_task_mask(tasks))?;
        Ok(Resource::new(resource, id))
    })
}
