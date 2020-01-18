//! Kernel module and routines declaration. `Private`

pub mod task;

#[cfg(any(feature = "events_32", feature = "events_16", feature = "events_64"))]
pub mod event;