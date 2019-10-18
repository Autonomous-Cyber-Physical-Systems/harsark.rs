#![no_std]
#![feature(asm)]
#![feature(const_fn)]

mod containers;
mod kernel;

pub mod event;
pub mod interrupts;
pub mod macros;
pub mod process;
pub mod sync;
pub use containers::*;

use crate::errors::KernelError;
use core::fmt;

pub mod helper {
    pub use crate::kernel::helper::generate_task_mask;
    pub use crate::kernel::helper::get_msb;
}

pub mod types {
    pub use crate::kernel::types::*;
}

mod config {
    pub const MAX_TASKS: usize = 32;
    pub const MAX_RESOURCES: usize = 32;
    pub const SEMAPHORE_COUNT: usize = 32;
    pub const MCB_COUNT: usize = 32;
    pub const MAX_BUFFER_SIZE: usize = 32;
    pub const EVENT_NO: usize = 32;
    pub const EVENT_INDEX_TABLE_COUNT: usize = 8;
    pub const MAX_STACK_SIZE: usize = 256;

    pub const OPCODE_SIGNAL: u8 = 1;
    pub const OPCODE_SEND_MSG: u8 = 1 << 1;
    pub const OPCODE_RELEASE: u8 = 1 << 2;
    pub const OPCODE_ENABLE_EVENT: u8 = 1 << 3;
}

pub mod errors {
    pub enum KernelError {
        BufferOverflow,
        NotFound,
        StackTooSmall,
        DoesNotExist,
        LimitExceeded,
        AccessDenied,
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
            KernelError::AccessDenied => write!(f, "AccessDenied"),
        }
    }
}
