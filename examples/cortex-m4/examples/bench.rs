#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::tasks::*;
use hartex_rust::helpers::TaskMask;
use hartex_rust::primitives::*;
use hartex_rust::spawn;

const task1: u32 = 1;
const task2: u32 = 2;
const task3: u32 = 3;

#[entry]
fn main() -> ! {
    let mut peripherals = cortex_m::Peripherals::take().unwrap();
    peripherals.DWT.enable_cycle_counter();

    static mut stack1: [u32; 128] = [0; 128];
    static mut stack2: [u32; 128] = [0; 128];
    static mut stack3: [u32; 128] = [0; 128];

    spawn!(task1, stack1, {
        hprintln!("TASK 1");
    });
    spawn!(task2, stack2, {
        cortex_m::asm::bkpt();
        hprintln!("TASK 2");
    });
    spawn!(task3, stack3, {
        hprintln!("TASK 3");
        cortex_m::asm::bkpt();
    });

    init();
    release(TaskMask::generate([task1, task2, task3]));
    start_kernel()
}
