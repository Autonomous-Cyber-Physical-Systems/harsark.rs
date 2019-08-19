#![no_std]

mod task_manager;

pub mod event_manager;
mod interrupt_handlers;
pub mod messaging;
pub mod resource_management;
pub mod semaphores;

pub mod tasks {
    pub use crate::task_manager::create_task;
    pub use crate::task_manager::init;
    pub use crate::task_manager::release_tasks;
    pub use crate::task_manager::start_kernel;
    pub use crate::task_manager::task_exit;
    pub use crate::task_manager::TaskId;
}

mod config {
    pub const MAX_TASKS: usize = 32;
    pub const SYSTICK_INTERRUPT_INTERVAL: u32 = 15_000;
    pub const SEMAPHORE_COUNT: usize = 32;
    pub const MCB_COUNT: usize = 32;
    pub const MAX_BUFFER_SIZE: usize = 32;
    pub const EVENT_NO: usize = 32;
    pub const EVENT_INDEX_TABLE_COUNT: usize = 8;
    pub const MAX_STACK_SIZE: usize = 128;
}

pub mod errors {
    pub enum KernelError {
        BufferOverflow,
        NotFound,
        StackTooSmall,
        DoesNotExist,
    }
}
