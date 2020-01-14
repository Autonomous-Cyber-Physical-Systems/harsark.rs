#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

// use hartex_rust::messages::Message;
use hartex_rust::resources;
use hartex_rust::semaphores::SemaphoreControlBlock;
use hartex_rust::spawn;
use hartex_rust::tasks::*;
use hartex_rust::types::*;
use hartex_rust::util::generate_task_mask;

struct AppState {
    sem3: SemaphoreControlBlock,
    msg1: Message<[u32; 2]>,
}

#[entry]
fn main() -> ! {
    let peripherals = resources::init_peripherals().unwrap();

    let app_inst = AppState {
        sem3: SemaphoreControlBlock::new(8),
        msg1: Message::new(
            4,
            4,
            [9, 10],
        ),
    };

    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];
    static mut stack3: [u32; 300] = [0; 300];

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

    init(true);
    release(generate_task_mask(&[task1]));
    start_kernel(
        unsafe { &mut peripherals.access().unwrap().borrow_mut() },
        150_000,
    );
    loop {}
}
