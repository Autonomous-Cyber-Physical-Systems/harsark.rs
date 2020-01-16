#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

// use hartex_rust::messages::Message;
use hartex_rust::resources;
use hartex_rust::events;
use hartex_rust::semaphores::SemaphoreControlBlock;
use hartex_rust::spawn;
use hartex_rust::tasks::*;
use hartex_rust::types::*;
use hartex_rust::util::*;

struct AppState {
    sem3: SemaphoreControlBlock,
    msg1: Message<[u32; 2]>,
}

#[entry]
fn main() -> ! {
    let peripherals = resources::init_peripherals();

    let app_inst = AppState {
        sem3: SemaphoreControlBlock::new(TaskMask::generate([3])),
        msg1: Message::new(
            TaskMask::generate([task2]),
            TaskMask::generate([task2]),
            [9, 10],
        )
        ,
    };

    let x = events::new(true, 2, || {
        hprintln!("Hello");
    });

    static mut stack1: [u32; 512] = [0; 512];
    static mut stack2: [u32; 512] = [0; 512];
    static mut stack3: [u32; 512] = [0; 512];

    spawn!(task1, 1, stack1, params, app_inst, {
        hprintln!("TASK 1: Enter");
        params.msg1.broadcast(Some([4, 5]));
        params.sem3.signal_and_release(0);
        hprintln!("TASK 1: END");
    });
    spawn!(task2, 2, stack2, params, app_inst, {
        hprintln!("TASK 2: Enter");
        params.msg1.receive(|msg| {
            hprintln!("TASK 2: msg received : {:?}", msg);
        });
        hprintln!("TASK 2: END");
    });
    spawn!(task3, 3, stack3, params, app_inst, {
        hprintln!("TASK 3: Enter");
        params.msg1.receive(|msg| {
            hprintln!("TASK 3: msg received : {:?}", msg);
        });
        hprintln!("TASK 3: END");
    });

    init(false);
    release(TaskMask::generate([task1]));
    peripherals.acquire(|peripherals| {
        events::systick_start(
            &mut peripherals.borrow_mut(),
            125_000_00,
        )
    });
    start_kernel();
}
