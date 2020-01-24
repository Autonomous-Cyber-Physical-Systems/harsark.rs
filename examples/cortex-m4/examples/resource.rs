#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::task::*;
use hartex_rust::util::TaskMask;
use hartex_rust::primitives::*;
use hartex_rust::spawn;

struct AppState {
    sem2: Semaphore,
    sem3: Semaphore,
    res1: Resource<[u32; 3]>,
    res2: Resource<[u32; 2]>,
}

const task1: u32 = 1;
const task2: u32 = 2;
const task3: u32 = 3;

#[entry]
fn main() -> ! {
    let peripherals = init_peripherals();

    static sem2: Semaphore = Semaphore::new(TaskMask::generate([task2]));
    static sem3: Semaphore = Semaphore::new(TaskMask::generate([task3]));
    static res1: Resource<[u32; 3]> = Resource::new([1, 2, 3], TaskMask::generate([task1, task2]));
    static res2: Resource<[u32; 2]> = Resource::new([4, 5], TaskMask::generate([task3]));

    static mut stack1: [u32; 512] = [0; 512];
    static mut stack2: [u32; 512] = [0; 512];
    static mut stack3: [u32; 512] = [0; 512];

    spawn!(task1, stack1, {
        hprintln!("TASK 1: Enter");
        // If res1 is free, then the closure passed on is executed on the resource.
        res1.acquire(|res| {
            hprintln!("TASK 1 : res1 : {:?}", res);
            sem2.signal_and_release(0);
            sem3.signal_and_release(0);
            for i in 0..10000 {}
            hprintln!("TASK 1 : task 2 and 3 dispatched");
        });
        hprintln!("TASK 1: End");
    });
    spawn!(task2, stack2, {
        hprintln!("TASK 2: Enter");
        res1.acquire(|res| {
            hprintln!("TASK 2 : res1 : {:?}", res);
        });
        hprintln!("TASK 2: End");
    });
    spawn!(task3, stack3, {
        hprintln!("TASK 3: Enter");
        res2.acquire(|res| {
            hprintln!("TASK 3 : res2 :  {:?}", res);
        });
        hprintln!("TASK 3: End");
    });

    init();
    release(TaskMask::generate([task1]));
    start_kernel()
}
