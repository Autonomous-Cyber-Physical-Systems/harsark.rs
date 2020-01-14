#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::tasks::*;
use hartex_rust::util::generate_task_mask;

use hartex_rust::resources;
use hartex_rust::semaphores::{self, SemaphoreControlBlock};
use hartex_rust::spawn;
use hartex_rust::types::*;

/*
The tasks can take only one argument, hence in case multiple variables have to be passed
then they must be encapsulated into a single struct.
*/
struct AppState {
    sem1: SemaphoreControlBlock,
    sem2: SemaphoreControlBlock,
}

#[entry]
fn main() -> ! {
    let peripherals = resources::init_peripherals().unwrap();

    /*
        Instance of AppState whose reference will be shared to all tasks.
        sem1 is a Semaphore that releases task1 on being signalled, similarly sem2 signals task2.
    */
    static app_inst: AppState = AppState {
        sem1: SemaphoreControlBlock::new(2),
        sem2: SemaphoreControlBlock::new(4),
    };

    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];
    static mut stack3: [u32; 300] = [0; 300];

    spawn!(task1, 1, stack1, params, app_inst, {
        hprintln!("TASK 1: Enter");
        params.sem2.signal_and_release(generate_task_mask(&[task2]));
        hprintln!("TASK 1: End");
    });

    spawn!(task2, 2, stack2, params, app_inst, {
        hprintln!("TASK 2: Enter");
        if params.sem2.test_and_reset().unwrap() {
            hprintln!("TASK 2: sem2 enabled");
        } else {
            hprintln!("TASK 2: sem2 disabled");
        }
        hprintln!("TASK 2: End");
    });

    spawn!(task3, 3, stack3, params, app_inst, {
        hprintln!("TASK 3: Enter");
        params.sem1.signal_and_release(0);
        hprintln!("TASK 3: End");
    });

    init(true);
    release(generate_task_mask(&[task2, task3]));
    start_kernel(
        unsafe { &mut peripherals.access().unwrap().borrow_mut() },
        150_000,
    );
    loop {}
}
