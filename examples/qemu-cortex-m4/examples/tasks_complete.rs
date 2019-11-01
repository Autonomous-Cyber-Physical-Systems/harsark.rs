#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::helper::generate_task_mask;
use hartex_rust::process::*;
use hartex_rust::resource::init_peripherals;
use hartex_rust::types::*;
use hartex_rust::{init, spawn};

#[entry]
fn main() -> ! {
    let peripherals = init_peripherals().unwrap();

    let task1_param = "Hello from task 1 !";
    let task2_param = "Hello from task 2 !";
    let task3_param = "Hello from task 3 !";
    let task_idle_param = "Waiting ...";

    static mut stack_idle: [u32; 300] = [0; 300];
    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];
    static mut stack3: [u32; 300] = [0; 300];

    spawn!(thread1, 1, stack1, param, task1_param, {
        hprintln!("{}", param);
    });
    spawn!(thread2, 2, stack2, param, task2_param, {
        hprintln!("{}", param);
    });
    spawn!(thread3, 3, stack3, param, task3_param, {
        hprintln!("{}", param);
    });

    init!(true, stack_idle, task_idle_param, |param| {
        hprintln!("{}", param);
        loop {}
    });
    release(generate_task_mask(&[1, 2, 3]));
    start_kernel(&mut peripherals.access().unwrap().borrow_mut(), 150_000);

    loop {}
}
