#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use harsark::events;
use harsark::helpers::TaskMask;
use harsark::primitives::*;
use harsark::spawn;
use harsark::tasks::*;
use harsark::timer;

const TASK1: u32 = 1;
const TASK2: u32 = 2;
const TASK3: u32 = 3;

const STACK_SIZE: usize = 512;

#[entry]
fn main() -> ! {
    let mut peripherals = cortex_m::Peripherals::take().unwrap();

    static sem2: Semaphore = Semaphore::new(TaskMask::generate([TASK2]));
    static msg1: Message<[u32; 2]> = Message::new(
        TaskMask::generate([TASK3]),
        TaskMask::generate([TASK3]),
        [9, 10],
    );

    let event1 = events::new(true, 3, || {
        msg1.broadcast([1, 2]);
    });
    let event2 = events::new(true, 2, || {
        sem2.signal_and_release(TaskMask::generate([TASK2]));
    });
    let event2 = events::new(true, 6, || {
        release(TaskMask::generate([TASK1]));
    });

    spawn!(
        TASK1,
        STACK_SIZE,
        (|cxt| {
            hprintln!("TASK 1: Enter");
            if let Ok(true) = sem2.test_and_reset(cxt) {
                hprintln!("TASK 1: sem2 enabled");
            }
            hprintln!("TASK 1: End");
        })
    );
    spawn!(
        TASK2,
        STACK_SIZE,
        (|cxt| {
            hprintln!("TASK 2: Enter");
            if let Ok(true) = sem2.test_and_reset(cxt) {
                hprintln!("TASK 2: sem2 enabled");
            }
            hprintln!("TASK 2: End");
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
            hprintln!("TASK 3: End");
        })
    );

    init(|_| Ok(()));
    timer::start_timer(&mut peripherals, 80_000_00);
    start_kernel()
}
