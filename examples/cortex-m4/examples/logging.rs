#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use harsark::helpers::TaskMask;
use harsark::logging;
use harsark::primitives::*;
use harsark::spawn;
use harsark::tasks::*;

const TASK1: u32 = 1;
const TASK2: u32 = 2;
const TASK3: u32 = 3;

const STACK_SIZE: usize = 512;

#[entry]
fn main() -> ! {
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
    spawn!(
        TASK1,
        STACK_SIZE,
        (|cxt| {
            hprintln!("TASK 1");
            logging::process(|log: logging::LogEvent| {
                hprintln!("{:?}", log);
            });
        })
    );
    spawn!(
        TASK2,
        STACK_SIZE,
        (|cxt| {
            hprintln!("TASK 2");
        })
    );
    spawn!(
        TASK3,
        STACK_SIZE,
        (|cxt| {
            hprintln!("TASK 3");
        })
    );

    // Initializes the kernel in preemptive mode.
    init(|_| Ok(()));

    // Releases tasks task1, task2, task3
    logging::set_all(true);
    release(TaskMask::generate([TASK1]));
    release(TaskMask::generate([TASK2]));
    release(TaskMask::generate([TASK3]));
    /*
    Starts scheduling tasks on the device.
    It requires a reference to the peripherals so as to start the SysTick timer.
    150_000 corresponds to the tick interval of the SysTick timer.
    */
    start_kernel()
}
