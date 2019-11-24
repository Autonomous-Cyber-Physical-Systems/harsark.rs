//! # Software Communication Module
//!
//! This module instantiates a global instance of MessageTable and defines Kernel Routines
//! and primitives which handle task communication.

use crate::utils::arch::is_privileged;
use crate::KernelError;

use crate::kernel::task_management::{get_curr_tid, release};
use crate::priv_execute;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::interrupt::{free as execute_critical, CriticalSection};

use crate::system::software_comm_bus::*;

use crate::system::types::MessageId;

/// Global instance of MessageTable
static MessageTable: Mutex<RefCell<MessagingManager>> =
    Mutex::new(RefCell::new(MessagingManager::new()));

/// It holds a variable of a generic type so that any message can be stored in it.
/// It designed to work without ending up in data races.
#[derive(Debug)]
pub struct Message<T: Sized> {
    /// Holds the Message that has to be sent to the receiver tasks.
    inner: RefCell<T>,
    /// Holds the MessageId corresponding to the message assigned by MessageTable.
    id: MessageId,
}

impl<T: Sized> Message<T> {
    /// Returns a new Instance of `Message` with the fields initialized to the values passed onto it as arguments.
    pub fn new(val: T, id: MessageId) -> Self {
        Self {
            inner: RefCell::new(val),
            id,
        }
    }

    /// Calls `broadcast()` on the `message_table` with `message_id` as `self.id`.
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

    /// Calls `receive()` on the `message_table` with `message_id` as `self.id`.
    /// If it returns true, then the message content (`self.inner`) is returned,
    /// else `None` is returned.
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

/// Broadcasts messages to all tasks specified during configuration. This function is used by event manager.
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

/// Returns an initialized Message Container.
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
