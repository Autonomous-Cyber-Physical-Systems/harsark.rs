#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use cortex_m::interrupt::Mutex;

use hartex_rust::process::*;
use hartex_rust::resource::init_peripherals;
use hartex_rust::types::*;
use hartex_rust::{init, spawn};
use hartex_rust::helper::generate_task_mask;

#[entry]
fn main() -> ! {
    let peripherals = init_peripherals().unwrap();

    let task1_param = "Hello from task 1 !";
    let task2_param = "Hello from task 2 !";
    let task3_param = "Hello from task 3 !";
    let task_init_param = "Waiting ...";

    spawn!(thread1, 1, param, task1_param, {
        hprintln!("{}",param);
    });
    spawn!(thread2, 2, param, task2_param,  {
        hprintln!("{}",param);
    });
    spawn!(thread3, 3, param, task3_param,  {
        hprintln!("{}",param);
    });

    init!(true, task_init_param, |param| {
        hprintln!("{}",param);
        loop {}
    });
    release(generate_task_mask(&[1,2,3]));
    start_kernel(&mut peripherals.access().unwrap().borrow_mut(), 150_000);

    loop {}
}