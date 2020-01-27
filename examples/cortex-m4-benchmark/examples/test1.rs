#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;
extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::task::*;
use hartex_rust::util::TaskMask;
use hartex_rust::primitives::*;
use hartex_rust::spawn;

const task1: u32 = 1;
const task2: u32 = 2;
const task3: u32 = 3;


#[entry]
fn main() -> ! {

    static mut stack1: [u32; 128] = [0; 128];
    static mut stack2: [u32; 128] = [0; 128];
    static mut stack3: [u32; 128] = [0; 128];
    
    // create_task(task1, unsafe {stack1}, || {
    //     loop{
    //         hprintln!("hello");
    //     }
    // });
    spawn!(task1, stack1, {
        hprintln!("TASK 1");
    });
    spawn!(task2, stack2, {
        hprintln!("TASK 2");
    });
    spawn!(task3, stack3, {
        hprintln!("TASK 3");
    });

    init();
    release(TaskMask::generate([task1, task2, task3]));
    start_kernel()
    // loop {}
}
