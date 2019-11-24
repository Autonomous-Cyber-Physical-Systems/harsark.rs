//! # Time Management Module.
//! Defines Kernel routines for time management.

use core::cell::RefCell;

use cortex_m::interrupt::free as execute_critical;
use cortex_m::interrupt::Mutex;

use crate::system::time_manager::{TickType, Time};

/// Global instance of Time manager.
static CURR_TIME: Mutex<RefCell<Time>> = Mutex::new(RefCell::new(Time::new()));

/// This method updates the Kernel of passing of 10 milliseconds.
pub fn tick() -> TickType {
    execute_critical(|cs_token| CURR_TIME.borrow(cs_token).borrow_mut().tick())
}

/// Returns the time elapsed since the starting of the application.
pub fn now() -> Time {
    execute_critical(|cs_token| CURR_TIME.borrow(cs_token).borrow().clone())
}

/// Returns the 10 Milliseconds field of current time.
pub fn get_msec_10() -> u32 {
    execute_critical(|cs_token| CURR_TIME.borrow(cs_token).borrow_mut().m_sec_10)
}
