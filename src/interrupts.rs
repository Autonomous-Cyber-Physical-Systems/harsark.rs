use crate::event::sweep_event_table;
use crate::internals::event_manager::EventTableType;
use crate::process::{is_preemptive, schedule};
use crate::config::PREEMPT_WAIT;
use cortex_m::interrupt::free as execute_critical;
use cortex_m_rt::exception;
use cortex_m_semihosting::hprintln;

static mut M_SEC: u32 = 0;
static mut SEC: u32 = 0;
static mut MIN: u32 = 0;

// SysTick Exception handler
#[exception]
fn SysTick() {
    execute_critical(|_| {
        let mut m_sec = unsafe { &mut M_SEC };
        let mut sec = unsafe { &mut SEC };
        let mut min = unsafe { &mut MIN };

        if *m_sec == PREEMPT_WAIT && is_preemptive() {
            schedule();
        }
        *m_sec += 1;

        let mut m_sec_flag = true;
        let mut sec_flag = false;
        let mut min_flag = false;
        let mut hour_flag = false;

        if *m_sec >= 100 {
            *sec += 1;
            *m_sec = 0;
            sec_flag = true;
        }

        if *sec >= 60 {
            *min += 1;
            *sec = 0;
            min_flag = true;
        }

        if *min >= 60 {
            *min = 0;
            hour_flag = true;
        }

        sweep_event_table(EventTableType::OnOff);
        if m_sec_flag {
            sweep_event_table(EventTableType::MilliSec);
        }
        if sec_flag {
            sweep_event_table(EventTableType::Sec);
        }
        if min_flag {
            sweep_event_table(EventTableType::Min);
        }
        if hour_flag {
            sweep_event_table(EventTableType::Hour);
        }
    });
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
