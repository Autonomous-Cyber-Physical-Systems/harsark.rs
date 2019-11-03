#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::util::generate_task_mask;
use hartex_rust::tasks::*;

use hartex_rust::types::*;
use hartex_rust::spawn;
use hartex_rust::resources;

#[entry]
fn main() -> ! {
    let peripherals = resources::init_peripherals().unwrap();

    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];
    static mut stack3: [u32; 300] = [0; 300];

    spawn!(task1, 1, stack1, {
        hprintln!("TASK 1");
    });
    spawn!(task2, 2, stack2, {
        hprintln!("TASK 2");
    });
    spawn!(task3, 3, stack3, {
        hprintln!("TASK 3");
    });

    init(true);
    release(generate_task_mask(&[task1, task2, task3]));

    start_kernel(unsafe {&mut peripherals.access().unwrap().borrow_mut()}, 150_000);

    loop {}
}
