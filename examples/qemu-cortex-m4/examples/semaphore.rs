#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use cortex_m::interrupt::Mutex;

use hartex_rust::process::*;
use hartex_rust::sync;
use hartex_rust::resource::init_peripherals;
use hartex_rust::types::*;
use hartex_rust::{init, spawn};
use hartex_rust::helper::generate_task_mask;

struct app {
    sem1: SemaphoreId,
    sem2: SemaphoreId
}

#[entry]
fn main() -> ! {
    let peripherals = init_peripherals().unwrap();

    let app_inst = app {
        sem1 : sync::create(generate_task_mask(&[1])).unwrap(),
        sem2 : sync::create(generate_task_mask(&[2])).unwrap()
    };

    spawn!(thread1, 1, params, app_inst, {
        hprintln!("task 1");
        sync::sem_set(params.sem2,generate_task_mask(&[2]));
    });
    spawn!(thread2, 2, params, app_inst, {
        hprintln!("task 2");
        if sync::sem_test(params.sem2).unwrap() {
            hprintln!("sem2 enabled");
        } else {
            hprintln!("sem2 disabled");
        }
    });
    spawn!(thread3, 3, params, app_inst, {
        hprintln!("task 3");
        sync::sem_set(params.sem1,0);
    });

    init!(true);
    release(generate_task_mask(&[2,3]));
    start_kernel(&mut peripherals.access().unwrap().borrow_mut(), 150_000);

    loop {}
}