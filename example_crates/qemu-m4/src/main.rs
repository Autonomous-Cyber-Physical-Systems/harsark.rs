#![no_std]
#![no_main]

extern crate panic_semihosting;
use cortex_m::interrupt::free;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use cortexm_threads::spawn;
use cortexm_threads::sync::{self, SemaphoreId};
use cortexm_threads::tasks::*;

#[entry]
fn main() -> ! {
    let sem1: SemaphoreId = sync::create(&[thread2]).unwrap();

    spawn!(thread2, 2, {
        for _ in 0..5 {
            let _ = hprintln!("in user task 2 !!");
        }
    });

    spawn!(thread1, 1, {
        for _ in 0..5 {
            let _ = hprintln!("in user task 1 !!");
        }
        sync::sem_post(0, &[thread2]);
    });

    release_tasks(&[1]);
    init(true);
    start_kernel();

    loop {}
}
