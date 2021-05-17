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

/*
The tasks can take only one argument, hence in case multiple variables have to be passed
then they must be encapsulated into a single struct.
*/

const TASK1: u32 = 1;
const TASK2: u32 = 2;
const TASK3: u32 = 3;

const STACK_SIZE: usize = 512;

#[entry]
fn main() -> ! {
    /*
        Instance of AppState whose reference will be shared to all TASKs.
        sem1 is a Semaphore that releases TASK1 on being signalled, similarly sem2 signals TASK2.
    */
    static sem1: Semaphore = Semaphore::new(TaskMask::generate([TASK1]));
    static sem2: Semaphore = Semaphore::new(TaskMask::generate([TASK2]));

    spawn!(
        TASK1,
        STACK_SIZE,
        (|cxt| {
            hprintln!("TASK 1: Enter");
            sem2.signal_and_release(TaskMask::generate([TASK2]));
            hprintln!("TASK 1: End");
        })
    );

    spawn!(
        TASK2,
        STACK_SIZE,
        (|cxt| {
            hprintln!("TASK 2: Enter");
            if sem2.test_and_reset(cxt).unwrap() {
                hprintln!("TASK 2: sem2 enabled");
            } else {
                hprintln!("TASK 2: sem2 disabled");
            }
            hprintln!("TASK 2: End");
        })
    );

    spawn!(
        TASK3,
        STACK_SIZE,
        (|cxt| {
            hprintln!("TASK 3: Enter");
            sem1.signal_and_release(0);
            hprintln!("TASK 3: End");
        })
    );

    init(|_| Ok(()));
    release(TaskMask::generate([TASK2, TASK3]));
    start_kernel()
}
