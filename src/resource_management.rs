use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use crate::kernel::resource_management::ResourceManager;

lazy_static!{
    static ref Resources: ResourceManager = ResourceManager::init();
}
