//! Task communication bus definition
//!
//! Inter task communication also utilizes semaphores to release tasks and keep track of the tasks
//! which can access the message and the tasks that have to be notified about the arrival of messages.

use core::cell::RefCell;

use crate::system::semaphore::Semaphore;
use crate::system::scheduler::BooleanVector;
use crate::utils::arch::critical_section;
use crate::kernel::tasks::get_curr_tid;

#[cfg(feature = "system_logger")]
use {
    crate::system::system_logger::LogEventType,
    crate::kernel::logging,
};

/// Holds details corresponding to a single message
pub struct Message<T: Sized + Clone> {
    /// Boolean vector representing the receiver tasks.
    value: RefCell<T>,
    pub receivers: BooleanVector,
    semaphore: Semaphore
}

impl<T: Sized + Clone> Message<T> {
    /// Creates a new entry in the `mcb_table` and `scb_table` corresponding to a message.
    pub const fn new(
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
    pub fn broadcast(&'static self,  msg: Option<T>) {
        critical_section(|_| {
            if let Some(msg) = msg {
                self.value.replace(msg);
            }
            self.semaphore.signal_and_release(self.receivers);
            #[cfg(feature = "system_logger")] {
                if logging::get_message_broadcast() {
                    logging::report(LogEventType::MessageBroadcast(self.receivers));
                }
            }
        })
    }

    pub fn receive (&'static self) -> Option<T>
    {
        critical_section(|_| {
            match self.semaphore.test_and_reset() {
                Ok(res) if res == true => {
                    #[cfg(feature = "system_logger")] {
                        if logging::get_message_recieve() {
                            logging::report(LogEventType::MessageRecieve(get_curr_tid() as u32));
                        }
                    }
                    Some(self.value.borrow().clone())
                },
                _ => None,
            }
        })
    }
}

unsafe impl<T: Sized + Clone> Sync for Message<T> {}
