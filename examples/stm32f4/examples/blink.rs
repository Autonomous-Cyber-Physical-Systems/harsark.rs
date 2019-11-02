#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32f4::stm32f407::{self, Peripherals};

use hartex_rust::event::{self, EventTableType, EventType};
use hartex_rust::helper::generate_task_mask;
use hartex_rust::message::{self, Message};
use hartex_rust::process::*;
use hartex_rust::resource::{self, Resource};
use hartex_rust::sync;
use hartex_rust::types::*;
use hartex_rust::spawn;

struct app{
    peripherals: Resource<RefCell<Peripherals>>,
}

fn peripherals_init(peripherals: &mut Peripherals) {
    let gpio = &peripherals.GPIOA;
    // RCC : reset and control clock
    let rcc = &peripherals.RCC;

    // the following is to enable clock for GPIOA
    rcc.ahb1enr.write(|w| w.gpioaen().set_bit());
    // enable timer
    rcc.apb1enr.write(|w| w.tim2en().set_bit());
// clear pin 6 config
    gpio
        .otyper
        .write(|w| w.ot6().clear_bit().ot7().clear_bit());

    // set LEDs D2(PA6), D3(PA7) as output
    gpio.moder.write(|w| w.moder6().output().moder7().output());

    // set pull_up mode for LEDs
    gpio
        .pupdr
        .write(|w| w.pupdr6().pull_up().pupdr7().pull_up());

}

#[entry]
fn main() -> ! {
    let peripherals = resource::init_peripherals().unwrap();

    let app_inst = app {
        peripherals: resource::create(RefCell::new(Peripherals::take().unwrap()), generate_task_mask(&[1,2])).unwrap()
    };

    app_inst.peripherals.acquire(|peripherals| {
        let peripherals = &mut peripherals.borrow_mut();
        peripherals_init(peripherals);
    });

    let e1 = event::create_FreeRunning(true, 2, EventTableType::Sec).unwrap();
    event::set_tasks(e1, generate_task_mask(&[1]));

    let e2 = event::create_FreeRunning(true, 3, EventTableType::Sec).unwrap();
    event::set_tasks(e2, generate_task_mask(&[2]));

    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];

    spawn!(thread1, 1, stack1, params, app_inst, {
        params.peripherals.acquire(|perf| {
            let perf = perf.borrow_mut();
            perf.GPIOA.odr.modify(|r, w| {
            let led2 = r.odr6().bit();
            if led2 {
                w.odr6().clear_bit()
            } else {
                w.odr6().set_bit()
            }
        });
        });
    });
    spawn!(thread2, 2, stack2, params, app_inst, {
        params.peripherals.acquire(|perf| {
            let perf = perf.borrow_mut();
            perf.GPIOA.odr.modify(|r, w| {
            let led3 = r.odr7().bit();
            if led3 {
                w.odr7().clear_bit()
            } else {
                w.odr7().set_bit()
            }
        });
        });
    });

    init(true);
    release(0);
    start_kernel(unsafe{&mut peripherals.access().unwrap().borrow_mut()}, 150_000);loop {}
}
