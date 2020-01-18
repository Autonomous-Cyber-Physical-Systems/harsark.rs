//! Task communication bus definition
//!
//! Inter task communication also utilizes semaphores to release tasks and keep track of the tasks
//! which can access the message and the tasks that have to be notified about the arrival of messages.

use crate::system::semaphore::Semaphore;
use crate::system::scheduler::BooleanVector;
use core::cell::RefCell;
use cortex_m::interrupt;

/// Holds details corresponding to a single message
pub struct Message<T> {
    /// Boolean vector representing the receiver tasks.
    value: RefCell<T>,
    pub receivers: BooleanVector,
    semaphore: Semaphore
}

impl<T: Clone> Message<T> {
    /// Creates a new entry in the `mcb_table` and `scb_table` corresponding to a message.
    pub fn new(
        tasks_mask: BooleanVector,
        receivers_mask: BooleanVector,
        value: T,
    ) -> Self {
        Self {
            value: RefCell::new(value),
            receivers: receivers_mask,
            semaphore: Semaphore::new(tasks_mask)
        }
    }

    /// The sender task calls this function, it broadcasts the message corresponding to `msg_id`.
    pub fn broadcast(&self,  msg: Option<T>) {
        interrupt::free(|_| {
            if let Some(msg) = msg {
                self.value.replace(msg);
            }
            self.semaphore.signal_and_release(self.receivers);
        })
    }

    pub fn receive<F,R> (&self, handler: F) -> Option<R>
    where
        F: Fn(&T) -> R, 
    {
        interrupt::free(|_| {
            match self.semaphore.test_and_reset() {
                Ok(res) if res == true => Some(handler(&self.value.borrow())),
                _ => None,
            }
        })
    }
}

unsafe impl<T> Sync for Message<T> {}
