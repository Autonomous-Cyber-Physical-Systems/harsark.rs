use crate::kernel::events::sweep_event_table;
use crate::system::event_manager::EventTableType;
use crate::system::time_manager::TickType;
use crate::kernel::time::{get_msec_10, tick};
use crate::kernel::tasks::{is_preemptive, schedule, preempt};
use cortex_m_rt::exception;
use cortex_m_semihosting::hprintln;

static mut M_SEC: u32 = 0;
static mut SEC: u32 = 0;
static mut MIN: u32 = 0;

// SysTick Exception handler
#[exception]
fn SysTick() {
    // hprintln!("{:?}", cortex_m::register::control::read().npriv());
    if is_preemptive() {
        schedule();
    }

    sweep_event_table(EventTableType::OnOff);
    sweep_event_table(EventTableType::MilliSec);

    match tick() {
        TickType::Hour => {
            sweep_event_table(EventTableType::Sec);
            sweep_event_table(EventTableType::Min);
            sweep_event_table(EventTableType::Hour);
        }
        TickType::Min => {
            sweep_event_table(EventTableType::Sec);
            sweep_event_table(EventTableType::Min);
        }
        TickType::Sec => {
            sweep_event_table(EventTableType::Sec);
        }
        _ => {}
    }
}

#[exception]
fn SVCall() {
    schedule();
}
