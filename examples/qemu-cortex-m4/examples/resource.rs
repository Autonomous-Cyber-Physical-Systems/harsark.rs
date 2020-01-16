#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::resources::{self,Resource};
use hartex_rust::semaphores::SemaphoreControlBlock;
use hartex_rust::spawn;
use hartex_rust::tasks::*;
use hartex_rust::types::*;
use hartex_rust::util::*;
use hartex_rust::util::{TaskMask,is_privileged};

struct AppState {
    sem2: SemaphoreControlBlock,
    sem3: SemaphoreControlBlock,
    res1: Resource<[u32; 3]>,
    res2: Resource<[u32; 2]>,
}

#[entry]
fn main() -> ! {
    let peripherals = resources::init_peripherals();

    // app_inst also holds the resource containers res1 and res2.
    let app_inst = AppState {
        sem2: SemaphoreControlBlock::new(TaskMask::generate([task2])),
        sem3: SemaphoreControlBlock::new(TaskMask::generate([task3])),
        res1: Resource::new([1, 2, 3], TaskMask::generate([task1, task2])),
        res2: Resource::new([4, 5], TaskMask::generate([task3])),
    };

    static mut stack1: [u32; 512] = [0; 512];
    static mut stack2: [u32; 512] = [0; 512];
    static mut stack3: [u32; 512] = [0; 512];
    // hprintln!("{:?}", cortex_m::register::control::read().npriv());

    spawn!(task1, 1, stack1, params, app_inst, {
        // hprintln!("{:?}", cortex_m::register::control::read().npriv());
        hprintln!("TASK 1: Enter");
        // If res1 is free, then the closure passed on is executed on the resource.
        params.res1.acquire(|res| {
            hprintln!("TASK 1 : res1 : {:?}", res);
            params.sem2.signal_and_release(0);
            params.sem3.signal_and_release(0);
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

    init(false);
    release(TaskMask::generate([task1]));
    start_kernel(
        unsafe { &mut peripherals.access().unwrap().borrow_mut() },
        150_000,
    );
    loop {}
}
