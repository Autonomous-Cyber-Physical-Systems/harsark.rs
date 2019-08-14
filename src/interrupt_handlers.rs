use crate::task_manager::{preempt, IS_PREEMPTIVE};
use cortex_m::interrupt::free as execute_critical;
use cortex_m_semihosting::hprintln;

static mut M_SEC: u32 = 0;
static mut SEC: u32 = 0;
static mut MIN: u32 = 0;

// SysTick Exception handler
#[no_mangle]
pub extern "C" fn SysTick() {
    execute_critical(|_| {
        if unsafe { IS_PREEMPTIVE } {
            preempt();
        }
        let mut m_sec = unsafe { &mut M_SEC };
        let mut sec = unsafe { &mut SEC };
        let mut min = unsafe { &mut MIN };

        *m_sec += 1;

        let mut m_sec_flag = true;
        let mut sec_flag = false;
        let mut min_flag = false;
        let mut hour_flag = false;

        if *m_sec >= 10 {
            *sec += 1;
            *m_sec = 0;
            sec_flag = true;
        }

        if *sec >= 5 {
            *min += 1;
            *sec = 0;
            min_flag = true;
        }

        if *min >= 5 {
            *min = 0;
            hour_flag = true;
        }
        hprintln!("{} {} {} {} ", m_sec, sec, min, hour_flag);
    });
}
