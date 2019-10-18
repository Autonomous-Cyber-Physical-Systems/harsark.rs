use core::fmt;

pub enum KernelError {
    BufferOverflow,
    NotFound,
    StackTooSmall,
    DoesNotExist,
    LimitExceeded,
    AccessDenied,
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
