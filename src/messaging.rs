//use core::alloc::
use crate::config::{MAX_BUFFER_SIZE, MAX_TASKS, MCB_COUNT, SEMAPHORE_COUNT};
use crate::errors::KernelError;
use crate::kernel::helper::check_priv;
use crate::kernel::semaphores::*;
use crate::process::{get_pid, release};
use cortex_m::interrupt::{free as execute_critical, CriticalSection};
use cortex_m::register::control::Npriv;
use cortex_m_semihosting::hprintln;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use crate::kernel::messaging::*;

use crate::kernel::types::MessageId;

static default_msg: [u32; 1] = [0; 1];

static Messaging: Mutex<RefCell<MessagingManager>> =
    Mutex::new(RefCell::new(MessagingManager::new()));

#[derive(Debug)]
pub struct Message<T: Sized> {
    inner: T,
    id: MessageId,
}

impl<T:Sized> Message<T> {
    pub fn new(val: T, id: MessageId) -> Self {
        Self {
            inner: val,
            id
        }
    }

    pub fn broadcast(&self) -> Result<(), KernelError> {
        execute_critical(|cs_token| {
            let mask = Messaging.borrow(cs_token).borrow_mut().broadcast(self.id)?;
            release(&mask)
        })
    }

    pub fn receive(&self) -> Option<&T> {
        execute_critical(|cs_token: &CriticalSection| {
            let mut msg = Messaging.borrow(cs_token).borrow_mut();
            if msg.receive(self.id, get_pid()) {
                return Some(&self.inner)
            }
            return None
        })
    }

}

pub fn create<T> (
    notify_tasks_mask: u32,
    receivers_mask: u32,
    msg: T
) -> Result<Message<T>, KernelError> 
where T: Sized {
    match check_priv() {
        Npriv::Unprivileged => Err(KernelError::AccessDenied),
        Npriv::Privileged => execute_critical(|cs_token| {
            let msg_id = Messaging.borrow(cs_token).borrow_mut().create(
                notify_tasks_mask,
                receivers_mask,
            )?;
            Ok(Message::new(msg, msg_id))
        }),
    }
}
