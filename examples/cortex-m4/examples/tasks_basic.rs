#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;

use harsark::events;
use harsark::helpers::TaskMask;
use harsark::primitives::*;
use harsark::spawn;
use harsark::tasks::*;
use harsark::KernelError;
// use harsark::logging;

#[entry]
fn main() -> ! {
    /*
    Gets an instance of cortex-m Peripherals struct wrapped in a RefCell inside a Resource container.
    Peripherals struct provides APIs to configure the hardware beneath.
    RefCell is used to provide interior mutability read more at :
    https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
    */
    let mut peripherals = cortex_m::Peripherals::take().unwrap();
    // peripherals.DWT.enable_cycle_counter();
    /*
    Define the task stacks corresponding to each task.
    Note to specify the stack size according to the task parameters and local variables etc.
    */

    /*
    Task definition.
    The first parameter corresponds to the name that will be used to refer to the task.
    The second variable corresponds to the priority of the task.
    The third variable corresponds to the task stack.
    The fourth variable corresponds to the task body.
    */

    const TASK1: u32 = 1;
    const TASK2: u32 = 2;
    const TASK3: u32 = 3;

    const STACK_SIZE: usize = 256;

    spawn!(
        TASK1,
        STACK_SIZE,
        (|cxt| {
            hprintln!("TASK 1: {:?}", cxt);
        })
    );
    spawn!(
        TASK2,
        STACK_SIZE,
        (|cxt| {
            hprintln!("TASK 2: {:?}", cxt);
        })
    );
    spawn!(
        TASK3,
        STACK_SIZE,
        (|cxt| {
            hprintln!("TASK 3: {:?}", cxt);
        })
    );

    // Initializes the kernel in preemptive mode.
    init(|_| Ok(()));

    // Releases tasks TASK1, TASK2, TASK3
    // logging::set_all(true);
    release(TaskMask::generate([TASK1]));
    // event::start_timer(&mut peripherals, 1000_0);
    /*
    Starts scheduling tasks on the device.
    It requires a reference to the peripherals so as to start the SysTick timer.
    150_000 corresponds to the tick interval of the SysTick timer.
    */
    start_kernel()
}

#[exception]
unsafe fn DefaultHandler(x: i16) -> ! {
    cortex_m::asm::bkpt();
    loop {}
}

#[exception]
unsafe fn UsageFault() -> ! {
    // prints the exception frame as a panic message
    cortex_m::asm::bkpt();
    loop {}
}
