//! Message primitive
//!

use core::cell::RefCell;

use crate::system::semaphore::Semaphore;
use crate::system::scheduler::BooleanVector;
use crate::utils::arch::critical_section;
use crate::tasks::get_curr_tid;

#[cfg(feature = "system_logger")]
use {
    crate::system::system_logger::LogEventType,
    crate::kernel::logging,
};

/// Holds metadata corresponding to a single message object.
pub struct Message<T: Sized + Clone> {
    value: RefCell<T>,
    pub receivers: BooleanVector,
    semaphore: Semaphore
}

impl<T: Sized + Clone> Message<T> {
    /// Create and initialize new message object
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

    /// Broadcast the message to all reciever tasks
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

    /// Get a copy of the messsage on recieving a message
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
