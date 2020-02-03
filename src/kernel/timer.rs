use core::cell::RefCell;

use crate::KernelError;
use crate::priv_execute;
use crate::system::scheduler::*;
use crate::utils::arch::{svc_call,Mutex,critical_section};
use crate::utils::helpers::is_privileged;

use cortex_m_semihosting::hprintln;

static SystemTimer: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));
 
pub fn update_time() {
    critical_section(|cs_token| {
        let time = &mut *SystemTimer.borrow(cs_token).borrow_mut();
        *time += 1;
    })
}

pub fn get_time() -> u32 {
    critical_section(|cs_token| {
        return *SystemTimer.borrow(cs_token).borrow()
    })
}