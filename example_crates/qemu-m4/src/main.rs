#![no_std]
#![no_main]

extern crate panic_semihosting;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::interrupt::{enable,disable};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use cortexm_threads::{create_thread_with_config, init, sleep};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(80_000);
    syst.enable_counter();
    syst.enable_interrupt();

    unsafe {
        disable();
    }

    let mut stack1 = [0xDEADBEEF; 512];
    let mut stack2 = [0xDEADBEEF; 512];
    let mut stack3 = [0xDEADBEEF; 512];

    let _ = create_thread_with_config(&mut stack1, || loop {
        let _ = hprintln!("in user task 1 !!");
        sleep(10);
    },
    2
    );
    let _ = create_thread_with_config(&mut stack3, || loop {
        let _ = hprintln!("in user task 3 !!");
        sleep(20);
    },
    3
    );
    let _ = create_thread_with_config(
        &mut stack2,
        || loop {
            let _ = hprintln!("in user task 2 !!");
            sleep(30);
        },
        1,
    );

    unsafe {
        enable();
    }

    init();
}
