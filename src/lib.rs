#![no_std]

pub mod messaging;
pub mod resource_management;
pub mod semaphores;
pub mod task_manager;
pub mod errors;

mod config {
    pub const MAX_TASKS: usize = 32;
    pub const SYSTICK_INTERRUPT_INTERVAL: u32 = 80_000;
    pub const SEMAPHORE_COUNT: usize = 32;
    pub const MCB_COUNT: usize = 32;
    pub const MAX_BUFFER_SIZE: usize = 32;
}