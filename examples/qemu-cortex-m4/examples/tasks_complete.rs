#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::tasks::*;
use hartex_rust::util::generate_task_mask;

use hartex_rust::resources;
use hartex_rust::spawn;
use hartex_rust::types::*;

#[entry]
fn main() -> ! {
    let peripherals = resources::init_peripherals().unwrap();

    // These are task parameters, they are passed to the task when called
    let task1_param = "Hello from task 1 !";
    let task2_param = "Hello from task 2 !";

    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];

    /*
    The task definition here is different :
    arg 1 : task name, this will be used to address the task across the code
    arg 2 : priority of the task
    arg 3 : task stack
    arg 4 : this corresponds to by what name will the task body refer the task argument
    arg 5 : the task argument
    arg 6 : task body
    */
    spawn!(task1, 1, stack1, param, task1_param, {
        hprintln!("{}", param);
    });
    spawn!(task2, 2, stack2, param, task2_param, {
        hprintln!("{}", param);
    });

    init(true);
    release(generate_task_mask(&[task1, task2]));
    start_kernel(
        unsafe { &mut peripherals.access().unwrap().borrow_mut() },
        150_000,
    );

    loop {}
}
