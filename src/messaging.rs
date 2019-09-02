//use core::alloc::
use crate::config::{MAX_BUFFER_SIZE, MAX_TASKS, MCB_COUNT, SEMAPHORE_COUNT};
use crate::errors::KernelError;
use crate::kernel::semaphores::*;
use crate::kernel::task_manager::{get_RT, release};
use cortex_m::interrupt::{free as execute_critical, CriticalSection};
use cortex_m_semihosting::hprintln;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use crate::kernel::messaging::*;

static default_msg: [u32;1] = [0;1];

    static Messaging: Mutex<RefCell<MessagingManager>> =
        Mutex::new(RefCell::new(MessagingManager {
            tcb_table: [TCB::new(); MAX_TASKS],
            mcb_table: [MCB {
                receivers: 0,
                src_buffer: &[],
            }; SEMAPHORE_COUNT],
            msg_scb_table: Semaphores::new()
        }));


pub fn broadcast(sem_id: SemaphoreId) -> Result<(), KernelError> {
    execute_critical(|cs_token| {
        Messaging.borrow(cs_token).borrow_mut().broadcast(sem_id)
    })
}

pub fn receive(sem_id: SemaphoreId, buffer: &mut [u32]) -> usize {
    execute_critical(|cs_token: &CriticalSection| {
        let mut msg = Messaging.borrow(cs_token).borrow_mut();
        let msg = msg.receive(sem_id);
        if let Some(msg) = msg {
            for i in 0..msg.len() {
                buffer[i] = msg[i];
            }
            msg.len()
        } else {
            0
        }
    })
}

pub fn new(
    var: usize,
    tasks: &[u32],
    receivers: &[u32],
    src_buffer: StaticBuffer,
) -> Result<SemaphoreId, KernelError> {
    execute_critical(|cs_token| {
        Messaging
            .borrow(cs_token)
            .borrow_mut()
            .create(tasks, receivers, src_buffer)
    })
}
