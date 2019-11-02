use crate::event::sweep_event_table;
use crate::internals::event_manager::EventTableType;
use crate::internals::time::TickType;
use crate::kernel::time::{get_msec_10, tick};
use crate::process::{is_preemptive, schedule};
use cortex_m_rt::exception;

static mut M_SEC: u32 = 0;
static mut SEC: u32 = 0;
static mut MIN: u32 = 0;

// SysTick Exception handler
#[exception]
fn SysTick() {
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

pub fn svc_call() {
    unsafe {
        asm!("svc 1");
    }
}
