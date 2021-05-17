#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;
extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;
use cortex_m_semihosting::hprintln;

use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use stm32f4::stm32f407::interrupt;
use stm32f4::stm32f407::{self, Peripherals};

use harsark::helpers::TaskMask;
use harsark::primitives::*;
use harsark::spawn;
use harsark::tasks::*;

const TASK1: u32 = 1;
const TASK2: u32 = 2;
const TASK3: u32 = 3;
const TASK4: u32 = 4;

const STACK_SIZE: usize = 512;

lazy_static! {
    static ref board_peripherals: Resource<RefCell<Peripherals>> = Resource::new(
        RefCell::new(Peripherals::take().unwrap()),
        TaskMask::generate([1, 2]),
    );
    static ref perif_pr3: Semaphore = Semaphore::new(TaskMask::generate([TASK3]));
    static ref perif_pr4: Semaphore = Semaphore::new(TaskMask::generate([TASK3]));
}

fn peripherals_configure(peripherals: &mut Peripherals) {
    // instances of configuration registers
    let rcc = &peripherals.RCC;
    let gpioe = &peripherals.GPIOE;
    let gpioa = &peripherals.GPIOA;
    let syscfg = &peripherals.SYSCFG;
    let exti = &peripherals.EXTI;

    // Enables the GPIOA(for the LEDs) and GPIOE(for the Buttons)
    rcc.ahb1enr
        .modify(|_, w| w.gpioeen().set_bit().gpioaen().set_bit());

    // Enables the clock
    rcc.apb2enr.write(|w| w.syscfgen().set_bit());

    // Sets the button K1(PE3) to input and pull_up
    gpioe.otyper.modify(|_, w| w.ot3().clear_bit());
    gpioe.moder.modify(|_, w| w.moder3().input());
    gpioe.pupdr.modify(|_, w| w.pupdr3().pull_up());
    // configures the external interrupt 3 to listen on PE, the number 0b0100 specifies the E GPIO bank (taken from the reference manual)
    syscfg
        .exticr1
        .modify(|_, w| unsafe { w.exti3().bits(0b0100) });

    // Sets the button K0(PE4) to input and pull_up
    gpioe.otyper.modify(|_, w| w.ot4().clear_bit());
    gpioe.moder.modify(|_, w| w.moder4().input());
    gpioe.pupdr.modify(|_, w| w.pupdr4().pull_up());
    // configures the external interrupt 4 to listen on PE, the number 0b0100 specifies the E GPIO bank (taken from the reference manual)
    syscfg
        .exticr2
        .modify(|_, w| unsafe { w.exti4().bits(0b0100) });

    // unmask the external interrupt 3 and 4
    exti.imr.modify(|_, w| w.mr3().set_bit().mr4().set_bit());

    // trigger the external interrupts 3 and 4 on rising-edge
    exti.rtsr.modify(|_, w| w.tr3().set_bit().tr4().set_bit());

    // enable the interrupts
    unsafe {
        NVIC::unmask(stm32f407::Interrupt::EXTI3);
        NVIC::unmask(stm32f407::Interrupt::EXTI4);
    }

    // clear pin 6 config
    gpioa
        .otyper
        .write(|w| w.ot6().clear_bit().ot7().clear_bit());

    // set LEDs D2(PA6), D3(PA7) as output
    gpioa.moder.write(|w| w.moder6().output().moder7().output());

    // set pull_up mode for LEDs
    gpioa
        .pupdr
        .write(|w| w.pupdr6().pull_up().pupdr7().pull_up());
}

#[interrupt]
fn EXTI3() {
    perif_pr3.signal_and_release(TaskMask::generate([TASK3]));
}

#[interrupt]
fn EXTI4() {
    perif_pr4.signal_and_release(TaskMask::generate([TASK3]));
}

#[entry]
fn main() -> ! {
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

    spawn!(
        TASK3,
        STACK_SIZE,
        (|cxt| {
            if perif_pr3.test_and_reset(cxt).unwrap() {
                board_peripherals.acquire(cxt, |peripherals| {
                    let peripherals = &mut *peripherals.borrow_mut();
                    peripherals.EXTI.pr.write(|w| w.pr3().set_bit());
                });
                release(2);
            }
            if perif_pr4.test_and_reset(cxt).unwrap() {
                board_peripherals.acquire(cxt, |peripherals| {
                    let peripherals = &mut *peripherals.borrow_mut();
                    peripherals.EXTI.pr.write(|w| w.pr4().set_bit());
                });
                release(4);
            }
        })
    );

    init(|cxt| {
        board_peripherals.acquire(cxt, |perf| {
            let perf = &mut *perf.borrow_mut();
            peripherals_configure(perf);
        })
    });
    start_kernel()
}
