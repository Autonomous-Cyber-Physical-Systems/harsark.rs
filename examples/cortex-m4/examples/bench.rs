#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;

use cortex_m::asm::*;
use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use harsark::helpers::TaskMask;
use harsark::primitives::*;
use harsark::spawn;
use harsark::tasks::*;

#[entry]
fn main() -> ! {
    let mut peripherals = cortex_m::Peripherals::take().unwrap();
    peripherals.DWT.enable_cycle_counter();

    const TASK1: u32 = 1;
    const TASK2: u32 = 2;
    const TASK3: u32 = 3;

    const STACK_SIZE: usize = 512;

    static msg1: Message<[u32; 2]> = Message::new(
        TaskMask::generate([TASK2]),
        TaskMask::generate([TASK2]),
        [0; 2],
    );
    spawn!(
        TASK2,
        STACK_SIZE,
        (|cxt| {
            if let Some(x) = msg1.receive(cxt) {
                hprintln!("TASK 1 {:?}", x);
            }
        })
    );
    spawn!(
        TASK1,
        STACK_SIZE,
        (|cxt| {
            // hprintln!("TASK 2");
        })
    );
    spawn!(
        TASK3,
        STACK_SIZE,
        (|cxt| {
            // hprintln!("TASK 3");
        })
    );

    // Initializes the kernel in preemptive mode.
    init(|_| Ok(()));

    // Releases tasks TASK1, TASK2, TASK3
    // logging::set_all(true);
    release(TaskMask::generate([1]));
    // event::start_timer(&mut peripherals, 1000_0);
    /*
    Starts scheduling tasks on the device.
    It requires a reference to the peripherals so as to start the SysTick timer.
    150_000 corresponds to the tick interval of the SysTick timer.
    */
    start_kernel()
}
