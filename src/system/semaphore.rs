//! # Software synchronization bus definition
//!
use cortex_m::{asm::bkpt, interrupt::CriticalSection};

use crate::utils::arch::critical_section;
use crate::KernelError;
use crate::{
    kernel::tasks::{get_curr_tid, schedule},
    tasks::Context,
};
use crate::{
    kernel::tasks::{preempt, TaskManager},
    system::scheduler::BooleanVector,
};
use core::cell::RefCell;

#[cfg(feature = "system_logger")]
use {crate::kernel::logging, crate::system::system_logger::LogEventType};

/// Enables task synchronization and communication.
pub struct Semaphore {
    /// It is a boolean vector which represents the tasks notified by the semaphore.
    pub flags: RefCell<BooleanVector>,
    /// It is a boolean vector that corresponds to the tasks that are to be released by the semaphore on being signaled.
    pub tasks: BooleanVector,
}

impl Semaphore {
    /// Initializes a new semaphore instance.
    pub const fn new(tasks: BooleanVector) -> Self {
        Self {
            flags: RefCell::new(0),
            tasks,
        }
    }

    /// Signals the semaphore, all tasks specified in semaphore::flags can test for it and all tasks in semaphore::tasks are released
    pub fn signal_and_release(&'static self, tasks_mask: BooleanVector) {
        critical_section(|cs| {
            self.signal_and_release_with_cs(cs, tasks_mask);
        });
    }

    #[inline(always)]
    pub(crate) fn signal_and_release_with_cs(
        &'static self,
        cs: &CriticalSection,
        tasks_mask: BooleanVector,
    ) {
        let flags: &mut BooleanVector = &mut self.flags.borrow_mut();
        *flags |= tasks_mask;
        let mut handle = TaskManager.borrow(cs).borrow_mut();
        handle.release(self.tasks);
        #[cfg(feature = "system_logger")]
        {
            if logging::get_semaphore_signal() {
                logging::report(LogEventType::SemaphoreSignal(*flags, self.tasks));
            }
        }
        schedule(handle.is_preemptive);
    }

    /// Checks if the flag was enabled for the currently running task.
    pub fn test_and_reset(&'static self, cxt: &Context) -> Result<bool, KernelError> {
        critical_section(|_| self.unsafe_test_and_reset(cxt))
    }

    /// Checks if the flag was enabled for the currently running task.
    pub(crate) fn unsafe_test_and_reset(&'static self, cxt: &Context) -> Result<bool, KernelError> {
        let flags: &mut BooleanVector = &mut self.flags.borrow_mut();
        let curr_tid = cxt.get_tid() as u32;
        let curr_tid_mask = 1 << curr_tid;
        if *flags & curr_tid_mask == curr_tid_mask {
            *flags &= !curr_tid_mask;
            #[cfg(feature = "system_logger")]
            {
                if logging::get_semaphore_reset() {
                    logging::report(LogEventType::SemaphoreReset(curr_tid));
                }
            }
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}

unsafe impl Sync for Semaphore {}
