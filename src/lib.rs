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
    pub use crate::kernel::messages::Message;
    pub use crate::kernel::resources::Resource;
    pub use crate::system::event_manager::{EventTableType, EventType};
    pub use crate::system::types::*;
}

/// Kernel routines which assist in Event management.
pub mod events {
    pub use crate::kernel::events::enable_event;
    pub use crate::kernel::events::new_FreeRunning;
    pub use crate::kernel::events::new_OnOff;
    pub use crate::kernel::events::set_message;
    pub use crate::kernel::events::set_next_event;
    pub use crate::kernel::events::set_semaphore;
    pub use crate::kernel::events::set_tasks;
}

/// Kernel routines which assist in Inter-task Communication.
pub mod messages {
    pub use crate::kernel::messages::new;
}

/// Kernel routines which assist in Resource management.
pub mod resources {
    pub use crate::kernel::resources::init_peripherals;
    pub use crate::kernel::resources::new;
}

/// Kernel routines which assist in Inter-task Synchronization.
pub mod semaphores {
    pub use crate::kernel::semaphores::new;
    pub use crate::kernel::semaphores::signal_and_release;
    pub use crate::kernel::semaphores::test_and_reset;
}

/// Kernel routines which assist in Task management.
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

/// Kernel routines which assist in Time management.
pub mod time {
    pub use crate::kernel::time::*;
}

#[cfg(feature = "alloc")]
pub use crate::utils::heap;
