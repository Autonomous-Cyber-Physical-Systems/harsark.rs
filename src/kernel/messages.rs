//use core::alloc::

use crate::utils::arch::is_privileged;
use crate::KernelError;

use crate::kernel::tasks::{get_curr_tid, release};
use crate::priv_execute;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::interrupt::{free as execute_critical, CriticalSection};
use cortex_m::register::control::Npriv;

use crate::system::software_comm_bus::*;

use crate::system::types::MessageId;

static MessageTable: Mutex<RefCell<MessagingManager>> =
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
            let mask = MessageTable
                .borrow(cs_token)
                .borrow_mut()
                .broadcast(self.id)?;
            release(mask);
            Ok(())
        })
    }

    pub fn receive(&self) -> Option<core::cell::Ref<'_, T>> {
        execute_critical(|cs_token: &CriticalSection| {
            let mut msg = MessageTable.borrow(cs_token).borrow_mut();
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
        let mask = MessageTable
            .borrow(cs_token)
            .borrow_mut()
            .broadcast(msg_id)?;
        release(mask);
        Ok(())
    })
}

pub fn new<T>(
    notify_tasks_mask: u32,
    receivers_mask: u32,
    msg: T,
) -> Result<Message<T>, KernelError>
where
    T: Sized,
{
    priv_execute!({
        execute_critical(|cs_token| {
            let msg_id = MessageTable
                .borrow(cs_token)
                .borrow_mut()
                .create(notify_tasks_mask, receivers_mask)?;
            Ok(Message::new(msg, msg_id))
        })
    })
}

unsafe impl<T> Sync for Message<T> {}
