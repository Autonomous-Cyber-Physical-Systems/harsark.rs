#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use harsark::helpers::TaskMask;
use harsark::primitives::*;
use harsark::spawn;
use harsark::tasks::*;

const TASK1: u32 = 1;
const TASK2: u32 = 2;
const TASK3: u32 = 3;

const STACK_SIZE: usize = 512;

#[entry]
fn main() -> ! {
    static sem3: Semaphore = Semaphore::new(TaskMask::generate([3]));
    static msg1: Message<[u32; 2]> = Message::new(
        TaskMask::generate([TASK2]),
        TaskMask::generate([TASK2]),
        [9, 10],
    );

    spawn!(
        TASK1,
        STACK_SIZE,
        (|cxt| {
            hprintln!("TASK 1: Enter");
            msg1.broadcast([4, 5]);
            sem3.signal_and_release(0);
            hprintln!("TASK 1: END");
        })
    );
    spawn!(
        TASK2,
        STACK_SIZE,
        (|cxt| {
            hprintln!("TASK 2: Enter");
            let msg = msg1.receive(cxt);
            if let Some(msg) = msg {
                hprintln!("TASK 2: msg received : {:?}", msg);
            }
            hprintln!("TASK 2: END");
        })
    );
    spawn!(
        TASK3,
        STACK_SIZE,
        (|cxt| {
            hprintln!("TASK 3: Enter");
            let msg = msg1.receive(cxt);
            if let Some(msg) = msg {
                hprintln!("TASK 3: msg received : {:?}", msg);
            }
            hprintln!("TASK 3: END");
        })
    );

    init(|_| Ok(()));
    release(TaskMask::generate([TASK1]));
    start_kernel()
}
