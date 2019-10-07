//use core::alloc::
use crate::config::{MAX_BUFFER_SIZE, MAX_TASKS, MCB_COUNT, SEMAPHORE_COUNT};
use crate::errors::KernelError;
use crate::kernel::semaphores::*;
use crate::process::{get_pid, release};
use cortex_m::interrupt::{free as execute_critical, CriticalSection};
use cortex_m_semihosting::hprintln;
use crate::kernel::helper::check_priv;
use cortex_m::register::control::Npriv;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use crate::kernel::messaging::*;

use crate::kernel::types::MessageId;

static default_msg: [u32; 1] = [0; 1];

static Messaging: Mutex<RefCell<MessagingManager>> = Mutex::new(RefCell::new(MessagingManager::new()));

pub fn broadcast(sem_id: MessageId) -> Result<(), KernelError> {
    execute_critical(|cs_token| Messaging.borrow(cs_token).borrow_mut().broadcast(sem_id))
}

pub fn receive(sem_id: MessageId, buffer: &mut [u32]) -> usize {
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
    tasks_mask: u32,
    receivers_mask: u32,
    src_buffer: StaticBuffer,
) -> Result<MessageId, KernelError> {
    match check_priv() {
        Npriv::Unprivileged => {
            Err(KernelError::AccessDenied)
        },
        Npriv::Privileged => {
            execute_critical(|cs_token| {
                Messaging
                    .borrow(cs_token)
                    .borrow_mut()
                    .create(tasks_mask, receivers_mask, src_buffer)
            })
        }
    }
}
