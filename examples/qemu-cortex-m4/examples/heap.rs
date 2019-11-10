#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;

extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;
use hartex_rust::alloc;
use alloc::vec::Vec;
use hartex_rust::heap::init_heap;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::util::generate_task_mask;
use hartex_rust::tasks::*;
use hartex_rust::resources;
use hartex_rust::semaphores;
use hartex_rust::types::*;
use hartex_rust::spawn;

lazy_static!{
    static ref resource1: Resource<RefCell<Vec<u32>>> = resources::new(
        RefCell::new(Vec::new()),
        generate_task_mask(&[1,2]))
        .unwrap();
}

#[entry]
fn main() -> ! {
    init_heap(50);
    let peripherals = resources::init_peripherals().unwrap();

    static mut stack1: [u32; 256] = [0; 256];
    static mut stack2: [u32; 256] = [0; 256];
    static mut stack3: [u32; 256] = [0; 256];

    spawn!(task1, 1, stack1, {
        hprintln!("TASK 1: Enter");
        resource1.acquire(|res| {
            let res = &mut res.borrow_mut();
            res.push(1);
            hprintln!("TASK 1: Resource : {:?}", res);
        });
        hprintln!("TASK 1: End");
    });
    spawn!(task2, 2, stack2, {
        hprintln!("TASK 2: Enter");
        resource1.acquire(|res| {
            let res = &mut res.borrow_mut();
            res.push(2);
            hprintln!("TASK 1: Resource : {:?}", res);
        });
        hprintln!("TASK 2: End");
    });

    init(true);
    release(generate_task_mask(&[task1, task2]));
    start_kernel(unsafe{&mut peripherals.access().unwrap().borrow_mut()}, 150_000);loop {}
}