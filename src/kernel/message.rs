//use core::alloc::

use crate::errors::KernelError;
use crate::internals::helper::is_privileged;

use crate::priv_execute;
use crate::process::{get_curr_tid, release};
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::interrupt::{free as execute_critical, CriticalSection};
use cortex_m::register::control::Npriv;

use crate::internals::messaging::*;

use crate::internals::types::MessageId;

static Messaging: Mutex<RefCell<MessagingManager>> =
    Mutex::new(RefCell::new(MessagingManager::new()));

#[derive(Debug)]
pub struct Message<T: Sized> {
    inner: RefCell<T>,
    id: MessageId,
}

impl<T: Sized> Message<T> {
    pub fn new(val: T, id: MessageId) -> Self {
        Self {
            inner: RefCell::new(val),
            id,
        }
    }

    pub fn broadcast(&self, msg: Option<T>) -> Result<(), KernelError> {
        execute_critical(|cs_token| {
            if let Some(msg) = msg {
                self.inner.replace(msg);
            }
            let mask = Messaging.borrow(cs_token).borrow_mut().broadcast(self.id)?;
            release(mask);
            Ok(())
        })
    }

    pub fn receive(&self) -> Option<core::cell::Ref<'_, T>> {
        execute_critical(|cs_token: &CriticalSection| {
            let mut msg = Messaging.borrow(cs_token).borrow_mut();
            if msg.receive(self.id, get_curr_tid()) {
                return Some(self.inner.borrow());
            }
            return None;
        })
    }

    pub fn get_id(&self) -> MessageId {
        self.id
    }
}

pub fn broadcast(msg_id: MessageId) -> Result<(), KernelError> {
    execute_critical(|cs_token| {
        let mask = Messaging.borrow(cs_token).borrow_mut().broadcast(msg_id)?;
        release(mask);
        Ok(())
    })
}

pub fn create<T>(
    notify_tasks_mask: u32,
    receivers_mask: u32,
    msg: T,
) -> Result<Message<T>, KernelError>
where
    T: Sized,
{
    priv_execute!({
        execute_critical(|cs_token| {
            let msg_id = Messaging
                .borrow(cs_token)
                .borrow_mut()
                .create(notify_tasks_mask, receivers_mask)?;
            Ok(Message::new(msg, msg_id))
        })
    })
}
