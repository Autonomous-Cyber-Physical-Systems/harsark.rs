#![no_std]
#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]

#[cfg(feature = "alloc")]
pub extern crate alloc;
#[cfg(feature = "alloc")]
extern crate alloc_cortex_m;

#[allow(non_upper_case_globals)]

#[macro_use]
extern crate cortex_m_rt;

mod config;
mod kernel;
mod system;
mod utils;

pub mod macros;

use crate::utils::errors::KernelError;

/// Helper functions.
pub mod util {
    pub use crate::utils::helpers::generate_task_mask;
}

/// Exports types defined across other Kernel modules.
pub mod types {
    pub use crate::kernel::software_comm_bus::Message;
    pub use crate::kernel::resource_management::Resource;
    pub use crate::system::event_manager::{EventTableType, EventType};
    pub use crate::system::types::*;
}

/// Kernel routines which assist in Event management.
pub mod events {
    pub use crate::kernel::event_management::enable_event;
    pub use crate::kernel::event_management::new_FreeRunning;
    pub use crate::kernel::event_management::new_OnOff;
    pub use crate::kernel::event_management::set_message;
    pub use crate::kernel::event_management::set_next_event;
    pub use crate::kernel::event_management::set_semaphore;
    pub use crate::kernel::event_management::set_tasks;
}

/// Kernel routines which assist in Inter-task Communication.
pub mod messages {
    pub use crate::kernel::software_comm_bus::new;
}

/// Kernel routines which assist in Resource management.
pub mod resources {
    pub use crate::kernel::resource_management::init_peripherals;
    pub use crate::kernel::resource_management::new;
}

/// Kernel routines which assist in Inter-task Synchronization.
pub mod semaphores {
    pub use crate::kernel::software_sync_bus::new;
    pub use crate::kernel::software_sync_bus::signal_and_release;
    pub use crate::kernel::software_sync_bus::test_and_reset;
}

/// Kernel routines which assist in Task management.
pub mod tasks {
    pub use crate::kernel::task_management::create_task;
    pub use crate::kernel::task_management::disable_preemption;
    pub use crate::kernel::task_management::enable_preemption;
    pub use crate::kernel::task_management::get_curr_tid;
    pub use crate::kernel::task_management::init;
    pub use crate::kernel::task_management::is_preemptive;
    pub use crate::kernel::task_management::release;
    pub use crate::kernel::task_management::start_kernel;
    pub use crate::kernel::task_management::task_exit;
}

/// Kernel routines which assist in Time management.
pub mod time {
    pub use crate::kernel::time_management::*;
}

#[cfg(feature = "alloc")]
pub use crate::utils::heap;
