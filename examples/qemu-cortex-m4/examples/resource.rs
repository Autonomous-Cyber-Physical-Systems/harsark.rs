#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::helper::generate_task_mask;
use hartex_rust::process::*;
use hartex_rust::resource::{self, Resource};
use hartex_rust::sync;
use hartex_rust::types::*;
use hartex_rust::spawn;

struct app {
    sem3: SemaphoreId,
    sem4: SemaphoreId,
    res1: Resource<[u32; 3]>,
    res2: Resource<[u32; 3]>,
}

#[entry]
fn main() -> ! {
    let peripherals = resource::init_peripherals().unwrap();

    let app_inst = app {
        sem3: sync::new(generate_task_mask(&[3])).unwrap(),
        sem4: sync::new(generate_task_mask(&[4])).unwrap(),
        res1: resource::new([1, 2, 3], generate_task_mask(&[1, 2, 3])).unwrap(),
        res2: resource::new([4, 5, 6], generate_task_mask(&[4])).unwrap(),
    };

    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];
    static mut stack3: [u32; 300] = [0; 300];
    static mut stack4: [u32; 300] = [0; 300];

    spawn!(thread1, 1, stack1, params, app_inst, {
        hprintln!("TASK 1: Enter");
        params.res1.acquire(|res| {
            hprintln!("TASK 1 : res1 : {:?}", res);
        });
        hprintln!("TASK 1: End");
    });
    spawn!(thread2, 2, stack2, params, app_inst, {
        hprintln!("TASK 2: Enter");
        params.res1.acquire(|res| {
            hprintln!("TASK 2 : res1 : {:?}", res);
            sync::signal_and_release(params.sem3, 0);
            sync::signal_and_release(params.sem4, 0);
            hprintln!("TASK 2 : task 3 and 4 dispatched");
        });
        hprintln!("TASK 2: End");
    });
    spawn!(thread3, 3, stack3, params, app_inst, {
        hprintln!("TASK 3: Enter");
        params.res1.acquire(|res| {
            hprintln!("TASK 3 : res1 : {:?}", res);
        });
        hprintln!("TASK 3: End");
    });
    spawn!(thread4, 4, stack4, params, app_inst, {
        hprintln!("TASK 4: Enter");
        params.res2.acquire(|res| {
            hprintln!("TASK 4 : res2 :  {:?}", res);
        });
        hprintln!("TASK 4: End");
    });

    init(true);
    release(generate_task_mask(&[1, 2]));
    start_kernel(unsafe{&mut peripherals.access().unwrap().borrow_mut()}, 150_000);loop {}
}
