#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::resources;
use hartex_rust::semaphores;
use hartex_rust::spawn;
use hartex_rust::tasks::*;
use hartex_rust::types::*;
use hartex_rust::util::generate_task_mask;

struct AppState {
    sem2: SemaphoreId,
    sem3: SemaphoreId,
    res1: Resource<[u32; 3]>,
    res2: Resource<[u32; 2]>,
}

#[entry]
fn main() -> ! {
    let peripherals = resources::init_peripherals().unwrap();

    // app_inst also holds the resource containers res1 and res2.
    let app_inst = AppState {
        sem2: semaphores::new(generate_task_mask(&[task2])).unwrap(),
        sem3: semaphores::new(generate_task_mask(&[task3])).unwrap(),
        res1: resources::new([1, 2, 3], generate_task_mask(&[task1, task2])).unwrap(),
        res2: resources::new([4, 5], generate_task_mask(&[task3])).unwrap(),
    };

    static mut stack1: [u32; 512] = [0; 512];
    static mut stack2: [u32; 512] = [0; 512];
    static mut stack3: [u32; 512] = [0; 512];

    spawn!(task1, 1, stack1, params, app_inst, {
        hprintln!("TASK 1: Enter");
        // If res1 is free, then the closure passed on is executed on the resource.
        params.res1.acquire(|res| {
            hprintln!("TASK 1 : res1 : {:?}", res);
            semaphores::signal_and_release(params.sem2, 0);
            semaphores::signal_and_release(params.sem3, 0);
            for i in 0..10000 {}
            hprintln!("TASK 1 : task 2 and 3 dispatched");
        });
        hprintln!("TASK 1: End");
    });
    spawn!(task2, 2, stack2, params, app_inst, {
        hprintln!("TASK 2: Enter");
        params.res1.acquire(|res| {
            hprintln!("TASK 2 : res1 : {:?}", res);
        });
        hprintln!("TASK 2: End");
    });
    spawn!(task3, 3, stack3, params, app_inst, {
        hprintln!("TASK 3: Enter");
        params.res2.acquire(|res| {
            hprintln!("TASK 3 : res2 :  {:?}", res);
        });
        hprintln!("TASK 3: End");
    });

    init(true);
    release(generate_task_mask(&[task1]));
    start_kernel(
        unsafe { &mut peripherals.access().unwrap().borrow_mut() },
        150_000,
    );
    loop {}
}
