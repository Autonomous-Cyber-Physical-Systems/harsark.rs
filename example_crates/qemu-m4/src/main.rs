#![no_std]
#![no_main]

extern crate panic_semihosting;
use cortex_m::interrupt::{disable, enable};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use cortexm_threads::task_manager::*;
use cortexm_threads::semaphores::*;
use cortexm_threads::messaging::*;

#[entry]
fn main() -> ! {
    disable();

    let mut stack1 = [0xDEADBEEF; 512];
    let mut stack2 = [0xDEADBEEF; 512];
    let mut stack3 = [0xDEADBEEF; 512];

    static mesg: [u32; 2]= [23, 34];

    configure_msg(1, &4, &4, &mesg);

    let _ = create_task(1, &mut stack1, || loop {
        for _ in 0..5 {
            let _ = hprintln!("in user task 1 !!");
        }
        broadcast(1);
    });
    let _ = create_task(2, &mut stack2, || loop {
        for _ in 0..5 {
            let _ = hprintln!("in user task 2 !!");
        }
        if let Some(msg) = receive(1) {
            hprintln!("abcdefghijklmnop {:?}", msg);
        }
    });
    let _ = create_task(3, &mut stack3, || loop {
        for _ in 0..5 {
            let _ = hprintln!("in user task 3 !!");
        }
    });

//    release(&2);

    unsafe {
        enable();
    }

//    init(false);
//    start_kernel();

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(80_000);
    syst.enable_counter();
    syst.enable_interrupt();

    loop {}
}
