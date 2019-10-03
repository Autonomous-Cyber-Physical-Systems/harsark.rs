#![no_std]
#![no_main]
#![feature(log_syntax)]

extern crate panic_halt;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;

use stm32f4::stm32f407;
use stm32f4::stm32f407::interrupt;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use hartex_rust::process::*;
use hartex_rust::spawn;
use hartex_rust::types::*;

#[entry]
fn main() -> ! {
    spawn!(thread1, 1, app, 6, {
        hprintln!("task 1  : {:?}", app);
    });
    spawn!(thread2, 2, app, 6, {
        hprintln!("task 2  : {:?}", app);
    });
    spawn!(thread3, 3, app, 6, {
        hprintln!("task 3  : {:?}", app);
    });

    init(true);
    release(&14);
    start_kernel();

    loop {}
}
