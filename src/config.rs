//! Kernel configuration.  `Private`

macro_rules! assert_unique_feature {
    () => {};
    ($first:tt $(,$rest:tt)*) => {
        $(
            #[cfg(all(feature = $first, feature = $rest))]
            compile_error!(concat!("features \"", $first, "\" and \"", $rest, "\" cannot be used together"));
        )*
        assert_unique_feature!($($rest),*);
    }
}

assert_unique_feature!("tasks_8","tasks_16","tasks_32");
assert_unique_feature!("events_64","events_16","events_32");
assert_unique_feature!("resources_64","resources_16","resources_32");

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


#[cfg(all(
    any(feature = "events_32",feature="default"),
    not(any(feature="events_16",feature="events_64"))
))]
pub const EVENT_COUNT: usize = 32;

#[cfg(all(feature = "events_16",not(feature="events_32")))]
pub const EVENT_COUNT: usize = 16;

#[cfg(all(feature = "events_64",not(feature="events_32")))]
pub const EVENT_COUNT: usize = 64;