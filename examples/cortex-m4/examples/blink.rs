#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;
extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;
use cortex_m_semihosting::hprintln;

use cortex_m_rt::entry;
use stm32f4::stm32f407::Peripherals;

use harsark::events;
use harsark::helpers::TaskMask;
use harsark::primitives::*;
use harsark::spawn;
use harsark::tasks::*;
use harsark::timer;
const TASK1: u32 = 1;
const TASK2: u32 = 2;

const STACK_SIZE: usize = 512;

lazy_static! {
    static ref board_peripherals: Resource<RefCell<Peripherals>> = Resource::new(
        RefCell::new(Peripherals::take().unwrap()),
        TaskMask::generate([1, 2]),
    );
}

fn peripherals_configure(peripherals: &mut Peripherals) {
    let gpio = &peripherals.GPIOA;
    // RCC : reset and control clock
    let rcc = &peripherals.RCC;

    // the following is to enable clock for GPIOA
    rcc.ahb1enr.write(|w| w.gpioaen().set_bit());
    // enable timer
    rcc.apb1enr.write(|w| w.tim2en().set_bit());
    // clear pin 6 config
    gpio.otyper.write(|w| w.ot6().clear_bit().ot7().clear_bit());

    // set LEDs D2(PA6), D3(PA7) as output
    gpio.moder.write(|w| w.moder6().output().moder7().output());

    // set pull_up mode for LEDs
    gpio.pupdr
        .write(|w| w.pupdr6().pull_up().pupdr7().pull_up());
}

#[entry]
fn main() -> ! {
    let mut cortex_peripherals = cortex_m::Peripherals::take().unwrap();

    let event1 = events::new(true, 5, || release(TaskMask::generate([TASK1])));
    let event2 = events::new(true, 4, || release(TaskMask::generate([TASK2])));

    spawn!(
        TASK1,
        STACK_SIZE,
        (|cxt| {
            board_peripherals.acquire(cxt, |perf| {
                let perf = &mut *perf.borrow_mut();
                perf.GPIOA.odr.modify(|r, w| {
                    let led2 = r.odr6().bit();
                    if led2 {
                        w.odr6().clear_bit()
                    } else {
                        w.odr6().set_bit()
                    }
                });
            });
        })
    );
    spawn!(
        TASK2,
        STACK_SIZE,
        (|cxt| {
            board_peripherals.acquire(cxt, |perf| {
                let perf = &mut *perf.borrow_mut();
                perf.GPIOA.odr.modify(|r, w| {
                    let led3 = r.odr7().bit();
                    if led3 {
                        w.odr7().clear_bit()
                    } else {
                        w.odr7().set_bit()
                    }
                });
            });
        })
    );

    init(|cxt| {
        board_peripherals.acquire(cxt, |perf| {
            let perf = &mut *perf.borrow_mut();
            peripherals_configure(perf);
        })
    });
    timer::start_timer(&mut cortex_peripherals, 80_000_00);
    start_kernel()
}
