pub const MAX_TASKS: usize = 32;
pub const MAX_RESOURCES: usize = 32;
pub const SEMAPHORE_COUNT: usize = 32;
pub const MCB_COUNT: usize = 32;
pub const MAX_BUFFER_SIZE: usize = 32;
pub const EVENT_NO: usize = 32;
pub const EVENT_INDEX_TABLE_COUNT: usize = 8;
pub const MAX_STACK_SIZE: usize = 400;

pub const OPCODE_SIGNAL: u8 = 1;
pub const OPCODE_SEND_MSG: u8 = 1 << 1;
pub const OPCODE_RELEASE: u8 = 1 << 2;
pub const OPCODE_ENABLE_EVENT: u8 = 1 << 3;
