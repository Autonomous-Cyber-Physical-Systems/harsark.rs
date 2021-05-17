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
use harsark::KernelError;

const TASK1: u32 = 1;
const TASK2: u32 = 2;
const TASK3: u32 = 3;

const STACK_SIZE: usize = 512;

#[entry]
fn main() -> ! {
    static sem2: Semaphore = Semaphore::new(TaskMask::generate([TASK2]));
    static sem3: Semaphore = Semaphore::new(TaskMask::generate([TASK3]));
    static res1: Resource<[u32; 3]> = Resource::new([1, 2, 3], TaskMask::generate([TASK1, TASK2]));
    static res2: Resource<[u32; 2]> = Resource::new([4, 5], TaskMask::generate([TASK3]));

    spawn!(
        TASK1,
        STACK_SIZE,
        (|cxt: &Context| {
            hprintln!("TASK 1: Enter");
            // If res1 is free, then the closure passed on is executed on the resource.
            res1.acquire(&cxt, |res| {
                hprintln!("TASK 1 : res1 : {:?}", res);
                sem2.signal_and_release(0);
                sem3.signal_and_release(0);
                for i in 0..10000 {}
                hprintln!("TASK 1 : TASK 2 and 3 dispatched");
            });
            hprintln!("TASK 1: End");
        })
    );
    spawn!(
        TASK2,
        STACK_SIZE,
        (|cxt: &Context| {
            hprintln!("TASK 2: Enter");
            res1.acquire(&cxt, |res| {
                hprintln!("TASK 2 : res1 : {:?}", res);
            });
            hprintln!("TASK 2: End");
        })
    );
    spawn!(
        TASK3,
        STACK_SIZE,
        (|cxt: &Context| {
            hprintln!("TASK 3: Enter");
            match res2.acquire(&cxt, |res| {
                return res[0] + res[1];
            }) {
                Ok(val) => {
                    hprintln!("Output: {:?}", val);
                }
                Err(KernelError::AccessDenied) => {
                    panic!("This TASK does not have access to the resource.");
                }
                Err(KernelError::LimitExceeded) => {
                    panic!("PiStack is full.");
                }
                Err(err) => {
                    panic!("Unexpected Error: {:?}", err);
                }
            }
            hprintln!("TASK 3: End");
        })
    );

    init(|_| Ok(()));
    release(TaskMask::generate([TASK1]));
    start_kernel()
}
