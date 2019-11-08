#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;


use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::util::generate_task_mask;
use hartex_rust::tasks::*;

use hartex_rust::semaphores;
use hartex_rust::resources;
use hartex_rust::types::*;
use hartex_rust::spawn;

struct app {
    sem1: SemaphoreId,
    sem2: SemaphoreId,
}

#[entry]
fn main() -> ! {
    let peripherals = resources::init_peripherals().unwrap();

    let app_inst = app {
        sem1: semaphores::new(generate_task_mask(&[task1])).unwrap(),
        sem2: semaphores::new(generate_task_mask(&[task2])).unwrap(),
    };

    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];
    static mut stack3: [u32; 300] = [0; 300];

    spawn!(task1, 1, stack1, params, app_inst, {
        hprintln!("TASK 1: Enter");
        semaphores::signal_and_release(params.sem2, generate_task_mask(&[task2]));
        hprintln!("TASK 1: End");
    });
    spawn!(task2, 2, stack2, params, app_inst, {
        hprintln!("TASK 2: Enter");
        if semaphores::test_and_reset(params.sem2).unwrap() {
            hprintln!("TASK 2: sem2 enabled");
        } else {
            hprintln!("TASK 2: sem2 disabled");
        }
        hprintln!("TASK 2: End");
    });
    spawn!(task3, 3, stack3, params, app_inst, {
        hprintln!("TASK 3: Enter");
        semaphores::signal_and_release(params.sem1, 0);
        hprintln!("TASK 3: End");
    });

    init(true);
    release(generate_task_mask(&[task2, task3]));
    start_kernel(unsafe{&mut peripherals.access().unwrap().borrow_mut()}, 150_000);loop {}
}
