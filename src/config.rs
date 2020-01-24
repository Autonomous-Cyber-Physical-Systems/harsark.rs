//! Kernel configuration.  `Private`

mod tasks_config {
    #[cfg(all(
        any(feature = "tasks_32", feature = "default"),
        not(any(feature = "tasks_16", feature = "tasks_8"))
    ))]
    pub const MAX_TASKS: usize = 32;

    #[cfg(feature = "tasks_16")]
    pub const MAX_TASKS: usize = 16;

    #[cfg(feature = "tasks_8")]
    pub const MAX_TASKS: usize = 8;

    #[cfg(any(
        all(feature = "tasks_32", any(feature = "tasks_16", feature = "tasks_8")),
        all(feature = "tasks_16", any(feature = "tasks_32", feature = "tasks_8")),
        all(feature = "tasks_8", any(feature = "tasks_32", feature = "tasks_16")),
    ))]
    compile_error!("Features 'tasks_32','tasks_18' and 'tasks_8' are mutually exclusive.");
}

mod resources_config {
    #[cfg(all(
        any(feature = "resources_32", feature = "default"),
        not(any(feature = "resources_16", feature = "resources_64"))
    ))]
    pub const MAX_RESOURCES: usize = 32;

    #[cfg(feature = "resources_16")]
    pub const MAX_RESOURCES: usize = 16;

    #[cfg(feature = "resources_64")]
    pub const MAX_RESOURCES: usize = 64;

    #[cfg(any(
        all(
            feature = "resources_32",
            any(feature = "resources_16", feature = "resources_64")
        ),
        all(
            feature = "resources_16",
            any(feature = "resources_32", feature = "resources_64")
        ),
        all(
            feature = "resources_64",
            any(feature = "resources_32", feature = "resources_16")
        ),
    ))]
    compile_error!(
        "Features 'resources_32','resources_18' and 'resources_64' are mutually exclusive."
    );
}

mod event_config {
    #[cfg(all(
        any(feature = "events_32", feature = "default"),
        not(any(feature = "events_16", feature = "events_64"))
    ))]
    pub const EVENT_COUNT: usize = 32;

    #[cfg(feature = "events_16")]
    pub const EVENT_COUNT: usize = 16;

    #[cfg(feature = "events_64")]
    pub const EVENT_COUNT: usize = 64;

    #[cfg(any(
        all(feature = "events_32", any(feature = "events_16", feature = "events_64")),
        all(feature = "events_16", any(feature = "events_32", feature = "events_64")),
        all(feature = "events_64", any(feature = "events_32", feature = "events_16")),
    ))]
    compile_error!("Features 'event_32','event_18' and 'event_64' are mutually exclusive.");
}

pub use resources_config::MAX_RESOURCES;
pub use tasks_config::MAX_TASKS;
pub use event_config::EVENT_COUNT;