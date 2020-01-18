#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::task::*;
use hartex_rust::util::TaskMask;
use hartex_rust::primitive::*;
use hartex_rust::spawn;

struct AppState {
    sem3: Semaphore,
    msg1: Message<[u32; 2]>,
}

const task1: u32 = 1;
const task2: u32 = 2;
const task3: u32 = 3;

#[entry]
fn main() -> ! {
    let peripherals = init_peripherals();

    let app_inst = AppState {
        sem3: Semaphore::new(TaskMask::generate([3])),
        msg1: Message::new(
            TaskMask::generate([task2]),
            TaskMask::generate([task2]),
            [9, 10],
        )
        ,
    };

    static mut stack1: [u32; 512] = [0; 512];
    static mut stack2: [u32; 512] = [0; 512];
    static mut stack3: [u32; 512] = [0; 512];

    spawn!(task1, stack1, params, app_inst, {
        hprintln!("TASK 1: Enter");
        params.msg1.broadcast(Some([4, 5]));
        params.sem3.signal_and_release(0);
        hprintln!("TASK 1: END");
    });
    spawn!(task2, stack2, params, app_inst, {
        hprintln!("TASK 2: Enter");
        params.msg1.receive(|msg| {
            hprintln!("TASK 2: msg received : {:?}", msg);
        });
        hprintln!("TASK 2: END");
    });
    spawn!(task3, stack3, params, app_inst, {
        hprintln!("TASK 3: Enter");
        params.msg1.receive(|msg| {
            hprintln!("TASK 3: msg received : {:?}", msg);
        });
        hprintln!("TASK 3: END");
    });

    init();
    release(TaskMask::generate([task1]));
    start_kernel()
}
