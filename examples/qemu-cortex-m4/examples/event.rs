#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::task::*;
use hartex_rust::event;
use hartex_rust::util::TaskMask;
use hartex_rust::primitive::*;
use hartex_rust::spawn;

const task1: u32 = 1;
const task2: u32 = 2;
const task3: u32 = 3;

#[entry]
fn main() -> ! {
    let peripherals = init_peripherals();
    
        static sem2: Semaphore = Semaphore::new(TaskMask::generate([task2]));
        static msg1: Message<[u32; 2]> = Message::new(
            TaskMask::generate([task3]),
            TaskMask::generate([task3]),
            [9, 10],
        );

    let event1 = event::new(true, 3, || {
        msg1.broadcast(Some([1,2]));
    });
    let event2 = event::new(true, 2, || {
        sem2.signal_and_release(TaskMask::generate([task2]));
    });
    let event2 = event::new(true, 6, || {
        release(TaskMask::generate([task1]));
    });

    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];
    static mut stack3: [u32; 300] = [0; 300];

    spawn!(task1, stack1, {
        hprintln!("TASK 1: Enter");
        if let Ok(true) = sem2.test_and_reset() {
            hprintln!("TASK 1: sem2 enabled");
        }
        hprintln!("TASK 1: End");
    });
    spawn!(task2, stack2, {
        hprintln!("TASK 2: Enter");
        if let Ok(true) = sem2.test_and_reset() {
            hprintln!("TASK 2: sem2 enabled");
        }
        hprintln!("TASK 2: End");
    });
    spawn!(task3, stack3, {
        hprintln!("TASK 3: Enter");
        msg1.receive(|msg| {
            hprintln!("TASK 3: msg received : {:?}", msg);
        });
        hprintln!("TASK 3: End");
    });

    init();
    release(TaskMask::generate([task1]));
    peripherals.acquire(|peripherals| {
        event::systick_start(
            &mut peripherals.borrow_mut(),
            80_000_00,
        )
    });
    start_kernel()
}
