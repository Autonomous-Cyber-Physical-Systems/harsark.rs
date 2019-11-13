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

    let task1_param = "Hello from task 1 !";
    let task2_param = "Hello from task 2 !";
    let task3_param = "Hello from task 3 !";
    let _task_idle_param = "Waiting ...";

    static mut stack_idle: [u32; 300] = [0; 300];
    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];
    static mut stack3: [u32; 300] = [0; 300];

    spawn!(task1, 1, stack1, param, task1_param, {
        hprintln!("{}", param);
    });
    spawn!(task2, 2, stack2, param, task2_param, {
        hprintln!("{}", param);
    });
    spawn!(task3, 3, stack3, param, task3_param, {
        hprintln!("{}", param);
    });

    init(true);
    release(generate_task_mask(&[task1, task2, task3]));
    start_kernel(
        unsafe { &mut peripherals.access().unwrap().borrow_mut() },
        150_000,
    );
    loop {}
}
