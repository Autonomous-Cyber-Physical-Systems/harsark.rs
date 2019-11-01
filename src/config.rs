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

mod semaphores_config {
    #[cfg(all(
        any(feature = "semahpores_32", feature = "default"),
        not(any(feature = "semahpores_16", feature = "semahpores_64"))
    ))]
    pub const SEMAPHORE_COUNT: usize = 32;

    #[cfg(feature = "semahpores_16")]
    pub const SEMAPHORE_COUNT: usize = 16;

    #[cfg(feature = "semahpores_64")]
    pub const SEMAPHORE_COUNT: usize = 64;

    #[cfg(any(
        all(
            feature = "semahpores_32",
            any(feature = "semahpores_16", feature = "semahpores_64")
        ),
        all(
            feature = "semahpores_16",
            any(feature = "semahpores_32", feature = "semahpores_64")
        ),
        all(
            feature = "semahpores_64",
            any(feature = "semahpores_32", feature = "semahpores_16")
        ),
    ))]
    compile_error!(
        "Features 'semahpores_32','semahpores_18' and 'semahpores_64' are mutually exclusive."
    );
}

mod message_config {
    #[cfg(all(
        any(feature = "message_32", feature = "default"),
        not(any(feature = "message_16", feature = "message_64"))
    ))]
    pub const MESSAGE_COUNT: usize = 32;

    #[cfg(feature = "message_16")]
    pub const MESSAGE_COUNT: usize = 16;

    #[cfg(feature = "message_64")]
    pub const MESSAGE_COUNT: usize = 64;

    #[cfg(any(
        all(
            feature = "message_32",
            any(feature = "message_16", feature = "message_64")
        ),
        all(
            feature = "message_16",
            any(feature = "message_32", feature = "message_64")
        ),
        all(
            feature = "message_64",
            any(feature = "message_32", feature = "message_16")
        ),
    ))]
    compile_error!("Features 'message_32','message_18' and 'message_64' are mutually exclusive.");
}

mod event_config {
    #[cfg(all(
        any(feature = "event_32", feature = "default"),
        not(any(feature = "event_16", feature = "event_64"))
    ))]
    pub const EVENT_COUNT: usize = 32;

    #[cfg(feature = "event_16")]
    pub const EVENT_COUNT: usize = 16;

    #[cfg(feature = "event_64")]
    pub const EVENT_COUNT: usize = 64;

    #[cfg(any(
        all(feature = "event_32", any(feature = "event_16", feature = "event_64")),
        all(feature = "event_16", any(feature = "event_32", feature = "event_64")),
        all(feature = "event_64", any(feature = "event_32", feature = "event_16")),
    ))]
    compile_error!("Features 'event_32','event_18' and 'event_64' are mutually exclusive.");
}

mod event_index_table_config {
    #[cfg(all(
        any(feature = "event_index_8", feature = "default"),
        not(feature = "event_index_16")
    ))]
    pub const EVENT_INDEX_TABLE_COUNT: usize = 8;

    #[cfg(feature = "event_16")]
    pub const EVENT_INDEX_TABLE_COUNT: usize = 16;

    #[cfg(any(all(feature = "event_index_8", feature = "event_index_16"),))]
    compile_error!("Features 'event_index_8' and 'event_index_16' are mutually exclusive.");
}

pub use message_config::MESSAGE_COUNT;
pub use resources_config::MAX_RESOURCES;
pub use semaphores_config::SEMAPHORE_COUNT;
pub use tasks_config::MAX_TASKS;

pub use event_config::EVENT_COUNT;
pub use event_index_table_config::EVENT_INDEX_TABLE_COUNT;

pub const PREEMPT_WAIT: u32 = 10;

pub const OPCODE_SIGNAL: u8 = 1;
pub const OPCODE_SEND_MSG: u8 = 1 << 1;
pub const OPCODE_RELEASE: u8 = 1 << 2;
pub const OPCODE_ENABLE_EVENT: u8 = 1 << 3;
