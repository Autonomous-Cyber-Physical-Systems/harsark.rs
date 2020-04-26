use core::cell::RefCell;

use crate::KernelError;
use crate::priv_execute;
use crate::system::scheduler::*;
use crate::utils::arch::{svc_call,Mutex,critical_section,SystClkSource,Peripherals};
use crate::utils::arch::is_privileged;

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

/// Starts the Kernel scheduler, which starts scheduling tasks and starts the SysTick timer using the
/// reference of the Peripherals instance and the `tick_interval`. `tick_interval` specifies the
/// frequency of the timer interrupt. The SysTick exception updates the kernel regarding the time
/// elapsed, which is used to dispatch events and schedule tasks.
pub fn start_timer(peripherals: &mut Peripherals, tick_interval: u32) {
    let syst = &mut peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(tick_interval);
    syst.enable_counter();
    syst.enable_interrupt();
}