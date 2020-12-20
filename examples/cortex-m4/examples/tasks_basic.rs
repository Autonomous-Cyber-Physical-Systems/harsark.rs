#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use harsark::tasks::*;
use harsark::helpers::{TaskMask};
use harsark::primitives::*;
use harsark::spawn;
use harsark::events;
// use harsark::logging;

const task1: u32 = 1;
const task2: u32 = 2;
const task3: u32 = 3;

static mut stack1: [u32; 128] = [0; 128];
static mut stack2: [u32; 128] = [0; 128];
static mut stack3: [u32; 128] = [0; 128];

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
    spawn!(task1, stack1, {
        hprintln!("TASK 1 {}", is_privileged());
    });
    spawn!(task2, stack2, {
        hprintln!("TASK 2");
    });
    spawn!(task3, stack3, {
        hprintln!("TASK 3");
    });


    // Initializes the kernel in preemptive mode.
    init();

    // Releases tasks task1, task2, task3
    // logging::set_all(true);
    release(TaskMask::generate([task1, task2, task3]));
    // event::start_timer(&mut peripherals, 1000_0);
    /*
    Starts scheduling tasks on the device.
    It requires a reference to the peripherals so as to start the SysTick timer.
    150_000 corresponds to the tick interval of the SysTick timer.
    */
    start_kernel()
}
