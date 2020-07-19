//! A safe and lightweight real-time Kernel written in Rust. Currently the kernel runs on cortex-m3/m4 
//! based microcontrollers, work is in progress on extending this to other platforms.
//! 
//! ## Usage
//! 
//! Place the following to the Cargo.toml:
//! 
//! ```toml
//! [dependencies]
//! harsark = { version = "0.3.4" }
//! ```

#![no_std]
#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(const_if_match)]
#![feature(const_loop)]
#![feature(const_generics)]

#[cfg(feature = "alloc")]
pub extern crate alloc;
#[cfg(feature = "alloc")]
extern crate alloc_cortex_m;

#[allow(non_upper_case_globals)]

extern crate cortex_m_rt;

mod config;
mod kernel;
mod system;
mod utils;

pub mod macros;

use crate::utils::errors::KernelError;

/// Helper functions.
pub mod helpers {
    pub use crate::utils::helpers::TaskMask;
    pub use crate::utils::arch::is_privileged;
}

/// Kernel routines which assist in Event management.
#[cfg(any(feature = "events_32", feature = "events_16", feature = "events_64"))]
pub mod events {
    pub use crate::kernel::events::enable;
    pub use crate::kernel::events::disable;
    pub use crate::kernel::events::new;
}

/// Kernel timer management.
#[cfg(feature = "timer")]
pub mod timer {
    pub use crate::kernel::timer::start_timer;
}
/// Kernel primitives which assist application development.
pub mod primitives {
    pub use crate::system::message::Message;
    pub use crate::system::resource::Resource;
    pub use crate::system::semaphore::Semaphore;
}

/// Kernel routines which assist in Task management.
pub mod tasks {
    pub use crate::kernel::tasks::enable_preemption;
    pub use crate::kernel::tasks::disable_preemption;
    pub use crate::kernel::tasks::create_task;
    pub use crate::kernel::tasks::init;
    pub use crate::kernel::tasks::get_curr_tid;
    pub use crate::kernel::tasks::release;
    pub use crate::kernel::tasks::start_kernel;
    pub use crate::kernel::tasks::task_exit;
}

#[cfg(feature="system_logger")]
/// Kernel routines which handle log management.
pub mod logging {
    pub use crate::kernel::logging::process;
    pub use crate::kernel::logging::set_all;
    pub use crate::kernel::logging::set_release;
    pub use crate::kernel::logging::set_block_tasks;
    pub use crate::kernel::logging::set_unblock_tasks;
    pub use crate::kernel::logging::set_task_exit;
    pub use crate::kernel::logging::set_resource_lock;
    pub use crate::kernel::logging::set_resource_unlock;
    pub use crate::kernel::logging::set_message_broadcast;
    pub use crate::kernel::logging::set_message_recieve;
    pub use crate::kernel::logging::set_semaphore_signal;
    pub use crate::kernel::logging::set_semaphore_reset;
    pub use crate::kernel::logging::set_timer_event;
    pub use crate::system::system_logger::LogEvent;
}

#[cfg(feature = "alloc")]
pub use crate::utils::heap;
