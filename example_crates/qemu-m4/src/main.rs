#![no_std]
#![no_main]

extern crate panic_semihosting;
use cortex_m::interrupt::{disable, enable};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use cortexm_threads::task_manager::*;
use cortexm_threads::semaphores::*;

#[entry]
fn main() -> ! {
    disable();

    let mut stack1 = [0xDEADBEEF; 512];
    let mut stack2 = [0xDEADBEEF; 512];
    let mut stack3 = [0xDEADBEEF; 512];

    semaphore_set_tasks(1, &[2]);

    let _ = create_task(1, &mut stack1, || loop {
        for _ in 0..5 {
            let _ = hprintln!("in user task 1 !!");
        }
        signal_and_release(1, &[3]);
    });
    let _ = create_task(2, &mut stack2, || loop {
        for _ in 0..5 {
            let _ = hprintln!("in user task 2 !!");
        }
    });
    let _ = create_task(3, &mut stack3, || loop {
        for _ in 0..5 {
            let _ = hprintln!("in user task 3 !!");
        }
    });

    let mut task_list = [false;32];
    task_list[1] = true;
    release(&task_list);

    unsafe {
        enable();
    }

    init(true);
    start_kernel();

    loop {}
}
