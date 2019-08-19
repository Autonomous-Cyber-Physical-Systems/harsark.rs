#![no_std]

pub mod errors;
pub mod event_manager;
mod interrupt_handlers;
pub mod messaging;
pub mod resource_management;
pub mod semaphores;
pub mod task_manager;

mod config {
    pub const MAX_TASKS: usize = 32;
    pub const SYSTICK_INTERRUPT_INTERVAL: u32 = 15_000;
    pub const SEMAPHORE_COUNT: usize = 32;
    pub const MCB_COUNT: usize = 32;
    pub const MAX_BUFFER_SIZE: usize = 32;
    pub const EVENT_NO: usize = 32;
    pub const EVENT_INDEX_TABLE_COUNT: usize = 8;
    pub const MAX_STACK_SIZE: usize = 512;
}
