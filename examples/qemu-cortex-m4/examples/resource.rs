#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use cortex_m::interrupt::Mutex;

use hartex_rust::process::*;
use hartex_rust::sync;
use hartex_rust::resource::{self, Resource};
use hartex_rust::types::*;
use hartex_rust::{init, spawn};
use hartex_rust::helper::generate_task_mask;

struct app {
    sem1: SemaphoreId,
    sem2: SemaphoreId,
    res1: Resource<u32>,
}

#[entry]
fn main() -> ! {
    let peripherals = resource::init_peripherals().unwrap();
    let app_inst = app {
        sem1 : sync::create(generate_task_mask(&[1])).unwrap(),
        sem2 : sync::create(generate_task_mask(&[2])).unwrap(),
        res1 : resource::create(9, generate_task_mask(&[1,2,3])).unwrap()
    };

    spawn!(thread1, 1, params, app_inst, {
        if let Some(x) = params.res1.lock() {
            hprintln!("task 1 {:?}", x);
            params.res1.unlock();
        }
    });
    spawn!(thread2, 2, params, app_inst, {
        if let Some(x) = params.res1.lock() {
            hprintln!("task 2 {:?}", x);
            params.res1.unlock();
        }
    });
    spawn!(thread3, 3, params, app_inst, {
        if let Some(x) = params.res1.lock() {
            hprintln!("task 3 {:?}", x);
            params.res1.unlock();
        }
    });

    init!(true);
    release(generate_task_mask(&[1,2,3]));
    start_kernel(&mut peripherals.access().unwrap().borrow_mut(), 150_000);

    loop {}
}