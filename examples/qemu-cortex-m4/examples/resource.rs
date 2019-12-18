#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

// #[macro_use]
// extern crate lazy_static;


use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::util::generate_task_mask;
use hartex_rust::tasks::*;
use hartex_rust::resources;
use hartex_rust::semaphores;
use hartex_rust::types::*;
use hartex_rust::spawn;

// lazy_static! {
//     static ref sem3: SemaphoreId = semaphores::new(generate_task_mask(&[3])).unwrap();
//     static ref sem4: SemaphoreId = semaphores::new(generate_task_mask(&[4])).unwrap();
//     static ref res1: Resource<[u32;3]> = resources::new([1, 2, 3], generate_task_mask(&[1, 2, 3])).unwrap();
//     static ref res2: Resource<[u32;3]> = resources::new([4, 5, 6], generate_task_mask(&[4])).unwrap();
// }


#[entry]
fn main() -> ! {
    let peripherals = resources::init_peripherals().unwrap();

    static mut stack1: [u32; 128] = [0; 128];
    static mut stack2: [u32; 128] = [0; 128];
    static mut stack3: [u32; 128] = [0; 128];
    static mut stack4: [u32; 128] = [0; 128];

    spawn!(task1, 1, stack1, {
        hprintln!("TASK 1: Enter");
        hprintln!("TASK 1: End");
    });
    spawn!(task2, 2, stack2, {
        hprintln!("TASK 2: Enter");
        hprintln!("TASK 2: End");
    });
    spawn!(task3, 3, stack3, {
        hprintln!("TASK 3: Enter");
        hprintln!("TASK 3: End");
    });
    spawn!(task4, 4, stack4, {
        hprintln!("TASK 4: Enter");
        hprintln!("TASK 4: End");
    });

    init(true);
    release(generate_task_mask(&[task2,task3,task4,task1]));
    start_kernel(unsafe{&mut peripherals.access().unwrap().borrow_mut()}, 150_000);loop {}
}
