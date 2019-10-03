#![no_std]
#![feature(asm)]
#![feature(const_fn)]

mod kernel;

pub mod containers;
pub mod event_manager;
pub mod interrupt_handlers;
pub mod macros;
pub mod messaging;
pub mod process;
pub mod sync;

use crate::errors::KernelError;
use core::fmt;

pub mod types {
    pub use crate::kernel::types::*;
}

mod config {
    pub const MAX_TASKS: usize = 32;
    pub const MAX_RESOURCES: usize = 32;
    pub const SYSTICK_INTERRUPT_INTERVAL: u32 = 15_000;
    pub const SEMAPHORE_COUNT: usize = 32;
    pub const MCB_COUNT: usize = 32;
    pub const MAX_BUFFER_SIZE: usize = 32;
    pub const EVENT_NO: usize = 32;
    pub const EVENT_INDEX_TABLE_COUNT: usize = 8;
    pub const MAX_STACK_SIZE: usize = 256;
}

pub mod errors {
    pub enum KernelError {
        BufferOverflow,
        NotFound,
        StackTooSmall,
        DoesNotExist,
        LimitExceeded,
    }
}

impl fmt::Debug for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            KernelError::DoesNotExist => write!(f, "DoesNotExist"),
            KernelError::BufferOverflow => write!(f, "BufferOverflow"),
            KernelError::NotFound => write!(f, "NotFound"),
            KernelError::StackTooSmall => write!(f, "StackTooSmall"),
            KernelError::LimitExceeded => write!(f, "LimitExceeded"),
        }
    }
}
