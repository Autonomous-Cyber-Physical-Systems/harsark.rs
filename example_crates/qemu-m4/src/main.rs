#![no_std]
#![no_main]

extern crate panic_semihosting;
use cortex_m::interrupt::{disable, enable};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use cortexm_threads::task_manager::*;

#[entry]
fn main() -> ! {
    disable();

    let mut stack1 = [0xDEADBEEF; 512];
    let mut stack2 = [0xDEADBEEF; 512];
    let mut stack3 = [0xDEADBEEF; 512];

    let _ = create_task(1, &mut stack1, || loop {
        for _ in 0..5 {
            let _ = hprintln!("in user task 1 !!");
        }
        block_unblock(1, true);
        block_unblock(2, false);
        block_unblock(3, true);
    });
    let _ = create_task(2, &mut stack2, || loop {
        for _ in 0..5 {
            let _ = hprintln!("in user task 2 !!");
        }
        block_unblock(1, true);
        block_unblock(2, true);
        block_unblock(3, false);
    });
    let _ = create_task(3, &mut stack3, || loop {
        for _ in 0..5 {
            let _ = hprintln!("in user task 3 !!");
        }
        block_unblock(1, false);
        block_unblock(2, true);
        block_unblock(3, true);
    });
    release(&[1, 2, 3]);

    unsafe {
        enable();
    }

    init(true);
    start_kernel();

    loop {}
}
