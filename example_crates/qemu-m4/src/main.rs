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
use cortexm_threads::spawn;
use cortexm_threads::task_manager::*;

#[entry]
fn main() -> ! {
    set_permitted_tasks(1,6);

    spawn!(thread1, 1, {
            for _ in 0..5 {
                let _ = hprintln!("in user task 1 !!");
            }
//            release(&4);
    });

    spawn!(thread2, 2, {
            for _ in 0..5 {
                let _ = hprintln!("in user task 2 !!");
            }
    });

    release_tasks(&[thread1]);
    init(true);
    start_kernel();

    loop {}
}
