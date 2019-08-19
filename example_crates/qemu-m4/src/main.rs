#![no_std]
#![no_main]

extern crate panic_semihosting;
use cortex_m::interrupt::{disable, enable};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use cortexm_threads::event_manager::*;
use cortexm_threads::messaging::*;
use cortexm_threads::resource_management::*;
use cortexm_threads::semaphores::*;
use cortexm_threads::task;
use cortexm_threads::task_manager::*;

#[entry]
fn main() -> ! {
    let mut stack1 = [0; 512];
    let mut stack2 = [0; 512];
    let mut stack3 = [0; 512];

    set_permitted_tasks(1, 6);

    let _ = task!(1, &mut stack1, {
        loop {
            for _ in 0..5 {
                let _ = hprintln!("in user task 1 !!");
            }
            lock(1);
            release(&4);
        }
    });
    let _ = task!(2, &mut stack2, {
        loop {
            for _ in 0..5 {
                let _ = hprintln!("in user task 2 !!");
            }
        }
    });

    release(&2);

    init(true);
    start_kernel();

    loop {}
}
