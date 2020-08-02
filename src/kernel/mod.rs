//! Kernel module and routines declaration. `Private`

pub mod tasks;

#[cfg(any(feature = "events_32", feature = "events_16", feature = "events_64"))]
pub mod events;

#[cfg(feature="system_logger")]
pub mod logging;

#[cfg(feature="process_monitor")]
pub mod task_monitor;

#[cfg(feature="timer")]
pub mod timer;