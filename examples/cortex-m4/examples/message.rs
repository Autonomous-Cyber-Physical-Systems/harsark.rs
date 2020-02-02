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

const task1: u32 = 1;
const task2: u32 = 2;
const task3: u32 = 3;

#[entry]
fn main() -> ! {
    let peripherals = init_peripherals();

        static sem3: Semaphore = Semaphore::new(TaskMask::generate([3]));
        static msg1: Message<[u32; 2]> = Message::new(
            TaskMask::generate([task2]),
            TaskMask::generate([task2]),
            [9, 10],
        );

    static mut stack1: [u32; 512] = [0; 512];
    static mut stack2: [u32; 512] = [0; 512];
    static mut stack3: [u32; 512] = [0; 512];

    spawn!(task1, stack1, {
        hprintln!("TASK 1: Enter");
        msg1.broadcast(Some([4, 5]));
        sem3.signal_and_release(0);
        hprintln!("TASK 1: END");
    });
    spawn!(task2, stack2, {
        hprintln!("TASK 2: Enter");
        let msg = msg1.receive();
        if let Some(msg) = msg {
            hprintln!("TASK 2: msg received : {:?}", msg);
        }
        hprintln!("TASK 2: END");
    });
    spawn!(task3, stack3, {
        hprintln!("TASK 3: Enter");
        let msg = msg1.receive();
        if let Some(msg) = msg {
            hprintln!("TASK 3: msg received : {:?}", msg);
        }
        hprintln!("TASK 3: END");
    });

    init();
    release(TaskMask::generate([task1]));
    start_kernel()
}
