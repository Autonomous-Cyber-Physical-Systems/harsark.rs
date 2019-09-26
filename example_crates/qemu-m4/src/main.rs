#![no_std]
#![no_main]

extern crate panic_halt;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;

use stm32f4::stm32f407;
use stm32f4::stm32f407::interrupt;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use hartex_rust::spawn;
use hartex_rust::sync::{self, SemaphoreId};
use hartex_rust::tasks::*;
use hartex_rust::{create_thread_with_config, init, sleep};

#[entry]
fn main() -> ! {
    // let sem1: SemaphoreId = sync::create(&[thread2]).unwrap();

    // spawn!(thread2, 2, {
    //     for _ in 0..5 {
    //         let _ = hprintln!("in user task 2 !!");
    //     }
    // });

    // spawn!(thread1, 1, {
    //     for _ in 0..5 {
    //         let _ = hprintln!("in user task 1 !!");
    //     }
    //     sync::sem_post(0, &[thread2]);
    // });

    // release_tasks(&[1]);
    // init(true);
    // start_kernel();
    // hartex_rust::interrupt_handlers::svc_call();
    loop {}
}
