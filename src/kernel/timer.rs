//! Manages the kernel timer.
use core::cell::RefCell;

use crate::KernelError;
use crate::priv_execute;
use crate::system::scheduler::*;
use crate::utils::arch::{svc_call,Mutex,critical_section,SystClkSource,Peripherals};
use crate::utils::arch::is_privileged;

static SystemTimer: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));
 
// TODO: on timer expire raise an event or make a log entry

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

/// Starts the Kernel timer. Timing event manager, logging and task monitor
/// are heavily dependent on the timer.
pub fn start_timer(peripherals: &mut Peripherals, tick_interval: u32) {
    let syst = &mut peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(tick_interval);
    syst.enable_counter();
    syst.enable_interrupt();
}