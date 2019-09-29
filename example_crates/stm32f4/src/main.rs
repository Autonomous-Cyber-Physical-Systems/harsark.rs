#![no_std]
#![no_main]

extern crate hartex_rust;
extern crate panic_halt;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;

use stm32f4::stm32f407;
use stm32f4::stm32f407::interrupt;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use hartex_rust::spawn;
use hartex_rust::sync;
use hartex_rust::tasks::*;
use hartex_rust::types::*;

static mut BOARD_PER: Option<stm32f407::Peripherals> = None;

#[entry]
fn main() -> ! {
    let board_peripherals = stm32f407::Peripherals::take().unwrap();

    // instances of configuration registers
    let rcc = &board_peripherals.RCC;
    let gpioe = &board_peripherals.GPIOE;
    let gpioa = &board_peripherals.GPIOA;
    let syscfg = &board_peripherals.SYSCFG;
    let exti = &board_peripherals.EXTI;

    // Enables the GPIOA(for the LEDs) and GPIOE(for the Buttons)
    rcc.ahb1enr
        .modify(|_, w| w.gpioeen().set_bit().gpioaen().set_bit());

    // Enables the clock
    rcc.apb2enr.write(|w| w.syscfgen().set_bit());
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

    // set the globals
    cortex_m::interrupt::free(|cs| unsafe {
        BOARD_PER.replace(board_peripherals);
    });

    let sem1: SemaphoreId = sync::create(&[thread3]).unwrap();

    spawn!(thread1, 1, {
        loop {
            cortex_m::asm::wfe();
        }
    });
    spawn!(thread2, 2, {
        for _ in 0..5 {
            let _ = hprintln!("in user task 2:1 !!");
        }
        sync::sem_post(0, &[thread3]);
        for _ in 0..5 {
            let _ = hprintln!("in user task 2:2 !!");
        }
        sync::sem_post(0, &[thread3]);
    });
    spawn!(thread3, 3, {
        for _ in 0..5 {
            let _ = hprintln!("in user task 3 !!");
        }
    });

    init(true);
    release_tasks(&[2, 3]);
    start_kernel();

    loop {}
}

fn fn1() {
    cortex_m::interrupt::free(|cs| unsafe {
        let perf = match &BOARD_PER {
            None => return,
            Some(v) => v,
        };
        perf.GPIOA.odr.modify(|r, w| {
            let led2 = r.odr6().bit();
            if led2 {
                w.odr6().clear_bit()
            } else {
                w.odr6().set_bit()
            }
        });
    });
}

fn fn2() {
    cortex_m::interrupt::free(|cs| unsafe {
        let perf = match &BOARD_PER {
            None => return,
            Some(v) => v,
        };
        perf.GPIOA.odr.modify(|r, w| {
            let led3 = r.odr7().bit();
            if led3 {
                w.odr7().clear_bit()
            } else {
                w.odr7().set_bit()
            }
        });
    });
}
