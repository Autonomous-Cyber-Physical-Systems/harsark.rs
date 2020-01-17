//! Error Enum

use core::fmt;

/// Error Type for Kernel.
pub enum KernelError {
    NotFound,
    StackTooSmall,
    LimitExceeded,
    AccessDenied,
    Empty,
    Exists,
}

impl fmt::Debug for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            KernelError::NotFound => write!(f, "NotFound"),
            KernelError::StackTooSmall => write!(f, "StackTooSmall"),
            KernelError::LimitExceeded => write!(f, "LimitExceeded"),
            KernelError::AccessDenied => write!(f, "AccessDenied"),
            KernelError::Empty => write!(f, "Empty"),
            KernelError::Exists => write!(f, "Exists"),
        }
    }
}
