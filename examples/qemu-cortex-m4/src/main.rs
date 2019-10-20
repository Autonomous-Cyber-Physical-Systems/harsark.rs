#![no_std]
#![no_main]
#![feature(log_syntax)]

extern crate panic_halt;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;

use stm32f4::stm32f407;
use stm32f4::stm32f407::interrupt;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use hartex_rust::helper::get_msb;
use hartex_rust::messaging;
use hartex_rust::process::*;
use hartex_rust::resource::*;
use hartex_rust::event;
use hartex_rust::sync;
use hartex_rust::event::{EventType,EventTableType};
use hartex_rust::types::*;
use hartex_rust::{init, spawn};

#[entry]
fn main() -> ! {
    let app = create(7, 14).unwrap();
    let peripherals = init_peripherals().unwrap();
    let msg1 = messaging::create(7,7,"hello").unwrap();
    let sm1 = sync::create(8).unwrap();

    let e2 = event::create(true, EventType::FreeRunning,1, EventTableType::Sec).unwrap();
    event::set_msg(e2,0);

    spawn!(thread1, 1, msg1, msg1, {
        hprintln!("task 1");
//        msg1.broadcast();
//        event::dispatch_event(*e1);
    });
    spawn!(thread2, 2, msg1, msg1, {
        hprintln!("task 2");
        if let Some(x) = msg1.receive() {
            hprintln!("{:?}", x);
        }
    });
    spawn!(thread3, 3, app, app, {
        hprintln!("task 3");
    });

    init!(true, &0, |_| loop {
        cortex_m::asm::wfe();
    });

    release(6);

    start_kernel(&mut peripherals.access().unwrap().borrow_mut(), 150_000);
    loop {}
}
