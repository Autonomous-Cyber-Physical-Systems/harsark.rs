//! Kernel configuration.  `Private`

#[cfg(all(
    any(feature = "tasks_32",feature="default"),
    not(any(feature="tasks_16",feature="tasks_8"))
))]
pub const MAX_TASKS: usize = 32;

#[cfg(all(feature = "tasks_16",not(feature="tasks_32")))]
pub const MAX_TASKS: usize = 16;

#[cfg(all(feature = "tasks_8",not(feature="tasks_32")))]
pub const MAX_TASKS: usize = 8;

#[cfg(all(
    any(feature = "resources_32",feature="default"),
    not(any(feature="resources_16",feature="resources_64"))
))]
pub const MAX_RESOURCES: usize = 32;

#[cfg(all(feature = "resources_16",not(feature="resources_32")))]
pub const MAX_RESOURCES: usize = 16;

#[cfg(all(feature = "resources_64",not(feature="resources_32")))]
pub const MAX_RESOURCES: usize = 64;


#[cfg(feature = "events_32")]
pub const EVENT_COUNT: usize = 32;

#[cfg(feature = "events_16")]
pub const EVENT_COUNT: usize = 16;

#[cfg(feature = "events_64")]
pub const EVENT_COUNT: usize = 64;