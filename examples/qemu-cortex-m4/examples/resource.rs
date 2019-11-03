#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::util::generate_task_mask;
use hartex_rust::tasks::*;
use hartex_rust::resources;
use hartex_rust::semaphores;
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
    let peripherals = resources::init_peripherals().unwrap();

    let app_inst = app {
        sem3: semaphores::new(generate_task_mask(&[task3])).unwrap(),
        sem4: semaphores::new(generate_task_mask(&[task4])).unwrap(),
        res1: resources::new([1, 2, 3], generate_task_mask(&[task1, task2, task3])).unwrap(),
        res2: resources::new([4, 5, 6], generate_task_mask(&[task4])).unwrap(),
    };

    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];
    static mut stack3: [u32; 300] = [0; 300];
    static mut stack4: [u32; 300] = [0; 300];

    spawn!(task1, 1, stack1, params, app_inst, {
        hprintln!("TASK 1: Enter");
        params.res1.acquire(|res| {
            hprintln!("TASK 1 : res1 : {:?}", res);
        });
        hprintln!("TASK 1: End");
    });
    spawn!(task2, 2, stack2, params, app_inst, {
        hprintln!("TASK 2: Enter");
        params.res1.acquire(|res| {
            hprintln!("TASK 2 : res1 : {:?}", res);
            semaphores::signal_and_release(params.sem3, 0);
            semaphores::signal_and_release(params.sem4, 0);
            hprintln!("TASK 2 : task 3 and 4 dispatched");
        });
        hprintln!("TASK 2: End");
    });
    spawn!(task3, 3, stack3, params, app_inst, {
        hprintln!("TASK 3: Enter");
        params.res1.acquire(|res| {
            hprintln!("TASK 3 : res1 : {:?}", res);
        });
        hprintln!("TASK 3: End");
    });
    spawn!(task4, 4, stack4, params, app_inst, {
        hprintln!("TASK 4: Enter");
        params.res2.acquire(|res| {
            hprintln!("TASK 4 : res2 :  {:?}", res);
        });
        hprintln!("TASK 4: End");
    });

    init(true);
    release(generate_task_mask(&[task1, task2]));
    start_kernel(unsafe{&mut peripherals.access().unwrap().borrow_mut()}, 150_000);loop {}
}
