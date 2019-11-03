use crate::system::time_manager::{TickType, Time};
use core::cell::RefCell;
use cortex_m::interrupt::free as execute_critical;
use cortex_m::interrupt::Mutex;

static CURR_TIME: Mutex<RefCell<Time>> = Mutex::new(RefCell::new(Time::new()));

pub fn tick() -> TickType {
    execute_critical(|cs_token| CURR_TIME.borrow(cs_token).borrow_mut().tick())
}

pub fn now() -> Time {
    execute_critical(|cs_token| CURR_TIME.borrow(cs_token).borrow().clone())
}

pub fn get_msec_10() -> u32 {
    execute_critical(|cs_token| CURR_TIME.borrow(cs_token).borrow_mut().m_sec_10)
}
