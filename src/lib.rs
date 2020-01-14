//! A safe and lightweight real-time Kernel written in Rust. The Kernel is developed for cortex-m3/m4 
//! based microcontrollers. The goal of the project was to develop a memory efficient, 
//! safe and lightweight real-time Kernel. Rust-lang was choosen due to its powerful compile-time checks.
//!  All the subsystems have been developed and manually tested. Language features like Traits, Generics 
//! helped define powerful and safe types. The Kernel uses statically allocated data structures to keep 
//! itself simple and fast. But using feature flags, the developer can enable dynamic memory allocation 
//! (for end-application and not Kernel itself). Cargo feature flags are used to configure constants 
//! such as maximum tasks, resources, etc. 
//! 
//! Features Flags : 
//! 
//! * alloc : Enables use of dynamic data-structures.
//! * tasks_8 : Max tasks in Kernel is set to 8.
//! * tasks_16 : Max tasks in Kernel is set to 16.
//! * tasks_32 : Max tasks in Kernel is set to 32.
//! * resources_16 : Max resources in Kernel is set to 16.
//! * resources_32 : Max resources in Kernel is set to 32.
//! * resources_64 : Max resources in Kernel is set to 64.
//! * messages_16 : Max messages in Kernel is set to 16.
//! * messages_32 : Max messages in Kernel is set to 32.
//! * messages_64 : Max messages in Kernel is set to 64.
//! * events_16 : Max events in Kernel is set to 16.
//! * events_32 : Max events in Kernel is set to 32.
//! * events_64 : Max events in Kernel is set to 64.

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
    pub use crate::system::software_comm_bus::Message;
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
    // pub use crate::kernel::software_comm_bus::new;
}

/// Kernel routines which assist in Resource management.
pub mod resources {
    pub use crate::kernel::resource_management::init_peripherals;
    pub use crate::kernel::resource_management::new;
}

/// Kernel routines which assist in Inter-task Synchronization.
pub mod semaphores {
    pub use crate::system::software_sync_bus::SemaphoreControlBlock;
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
