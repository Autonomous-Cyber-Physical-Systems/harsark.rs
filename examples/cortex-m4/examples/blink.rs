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

use hartex_rust::task::*;
use hartex_rust::util::TaskMask;
use hartex_rust::primitive::*;
use hartex_rust::spawn;
use hartex_rust::event;

const task1: u32 = 1;
const task2: u32 = 2;

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
    let cortex_peripherals = init_peripherals();

    board_peripherals.acquire(|perf| {
        let perf = &mut *perf.borrow_mut();
        peripherals_configure(perf);
    });

    let event1 = event::new(true, 3, || {
        release(TaskMask::generate([task1]))
    });
    let event2 = event::new(true, 4, || {
        release(TaskMask::generate([task2]))
    });

    static mut stack1: [u32; 512] = [0; 512];
    static mut stack2: [u32; 512] = [0; 512];

    spawn!(task1, stack1, {
        board_peripherals.acquire(|perf| {
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
    });
    spawn!(task2, stack2, {
        board_peripherals.acquire(|perf| {
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
    });

    init();
    release(3);
    cortex_peripherals.acquire(|perf| {
        let perf = &mut *perf.borrow_mut();
        event::systick_start(
            perf,
            80_000_00,
        )
    });
    start_kernel()
}
