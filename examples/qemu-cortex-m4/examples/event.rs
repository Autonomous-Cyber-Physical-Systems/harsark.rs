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
use hartex_rust::event::{self, EventType, EventTableType};
use hartex_rust::resource::{self, Resource};
use hartex_rust::types::*;
use hartex_rust::{init, spawn};
use hartex_rust::helper::generate_task_mask;

struct app {
    sem2: SemaphoreId,
}

#[entry]
fn main() -> ! {
    let peripherals = resource::init_peripherals().unwrap();

    let app_inst = app {
        sem2 : sync::create(generate_task_mask(&[2])).unwrap(),
    };

    let e1 = event::create(true, EventType::FreeRunning,2, EventTableType::Sec).unwrap();
    event::set_tasks(e1,generate_task_mask(&[1]));

    let e2 = event::create(true, EventType::FreeRunning,4, EventTableType::Sec).unwrap();
    event::set_semaphore(e2,app_inst.sem2,generate_task_mask(&[1,2]));

    spawn!(thread1, 1, params, app_inst, {
        hprintln!("TASK 1: Enter");
        if let Ok(x) = sync::sem_test(params.sem2) {
            if (x) {
                hprintln!("TASK 1: sem2 enabled");
            }
        }
        hprintln!("TASK 1: End");
    });
    spawn!(thread2, 2, params, app_inst, {
        hprintln!("TASK 2: Enter");
        if let Ok(x) = sync::sem_test(params.sem2) {
            hprintln!("TASK 2: sem2 enabled");
        }
        hprintln!("TASK 2: End");
    });
    spawn!(thread3, 3, params, app_inst, {
    });

    init!(true);
    release(0);
    start_kernel(&mut peripherals.access().unwrap().borrow_mut(), 150_000);

    loop {}
}