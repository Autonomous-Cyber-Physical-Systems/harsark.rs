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
use hartex_rust::sync;
use hartex_rust::types::*;
use hartex_rust::spawn;

struct app {
    sem1: SemaphoreId,
    sem2: SemaphoreId,
}

#[entry]
fn main() -> ! {
    let peripherals = init_peripherals().unwrap();

    let app_inst = app {
        sem1: sync::create(generate_task_mask(&[1])).unwrap(),
        sem2: sync::create(generate_task_mask(&[2])).unwrap(),
    };

    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];
    static mut stack3: [u32; 300] = [0; 300];

    spawn!(thread1, 1, stack1, params, app_inst, {
        hprintln!("TASK 1: Enter");
        sync::sem_set(params.sem2, generate_task_mask(&[2]));
        hprintln!("TASK 1: End");
    });
    spawn!(thread2, 2, stack2, params, app_inst, {
        hprintln!("TASK 2: Enter");
        if sync::sem_test(params.sem2).unwrap() {
            hprintln!("TASK 2: sem2 enabled");
        } else {
            hprintln!("TASK 2: sem2 disabled");
        }
        hprintln!("TASK 2: End");
    });
    spawn!(thread3, 3, stack3, params, app_inst, {
        hprintln!("TASK 3: Enter");
        sync::sem_set(params.sem1, 0);
        hprintln!("TASK 3: End");
    });

    init(true);
    release(generate_task_mask(&[2, 3]));
    start_kernel(&mut peripherals.access().unwrap().borrow_mut(), 150_000);

    loop {}
}
