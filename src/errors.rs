use core::fmt;

pub enum KernelError {
    NotFound,
    StackTooSmall,
    LimitExceeded,
    AccessDenied,
}

impl fmt::Debug for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            KernelError::NotFound => write!(f, "NotFound"),
            KernelError::StackTooSmall => write!(f, "StackTooSmall"),
            KernelError::LimitExceeded => write!(f, "LimitExceeded"),
            KernelError::AccessDenied => write!(f, "AccessDenied"),
        }
    }
}
