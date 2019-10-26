
#[cfg(all(any(feature = "tasks_32",feature = "default"), not(any(feature = "tasks_16",feature = "tasks_8"))))]
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

pub const MAX_RESOURCES: usize = 32;
pub const SEMAPHORE_COUNT: usize = 32;

pub const MAX_BUFFER_SIZE: usize = 32;
pub const EVENT_NO: usize = 32;
pub const EVENT_INDEX_TABLE_COUNT: usize = 8;
pub const MAX_STACK_SIZE: usize = 300;

pub const OPCODE_SIGNAL: u8 = 1;
pub const OPCODE_SEND_MSG: u8 = 1 << 1;
pub const OPCODE_RELEASE: u8 = 1 << 2;
pub const OPCODE_ENABLE_EVENT: u8 = 1 << 3;
