#![no_std]
#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]

#[cfg(feature = "alloc")]
pub extern crate alloc;
#[cfg(feature = "alloc")]
extern crate alloc_cortex_m;

#[macro_use]
extern crate cortex_m_rt;

#[allow(non_upper_case_globals)]
mod config;
mod kernel;
mod system;
mod utils;

pub mod macros;

use crate::utils::errors::KernelError;

pub mod util {
    pub use crate::utils::helpers::generate_task_mask;
}

pub mod types {
    pub use crate::kernel::messages::Message;
    pub use crate::kernel::resources::Resource;
    pub use crate::system::event_manager::{EventTableType, EventType};
    pub use crate::system::types::*;
}

pub mod events {
    pub use crate::kernel::events::enable_event;
    pub use crate::kernel::events::new_FreeRunning;
    pub use crate::kernel::events::new_OnOff;
    pub use crate::kernel::events::set_message;
    pub use crate::kernel::events::set_next_event;
    pub use crate::kernel::events::set_semaphore;
    pub use crate::kernel::events::set_tasks;
}

pub mod messages {
    pub use crate::kernel::messages::new;
}

pub mod resources {
    pub use crate::kernel::resources::init_peripherals;
    pub use crate::kernel::resources::new;
}

pub mod semaphores {
    pub use crate::kernel::semaphores::new;
    pub use crate::kernel::semaphores::signal_and_release;
    pub use crate::kernel::semaphores::test_and_reset;
}

pub mod tasks {
    pub use crate::kernel::tasks::create_task;
    pub use crate::kernel::tasks::disable_preemption;
    pub use crate::kernel::tasks::enable_preemption;
    pub use crate::kernel::tasks::get_curr_tid;
    pub use crate::kernel::tasks::init;
    pub use crate::kernel::tasks::is_preemptive;
    pub use crate::kernel::tasks::release;
    pub use crate::kernel::tasks::start_kernel;
    pub use crate::kernel::tasks::task_exit;
}

pub mod time {
    pub use crate::kernel::time::*;
}

#[cfg(feature = "alloc")]
pub use crate::utils::heap;
