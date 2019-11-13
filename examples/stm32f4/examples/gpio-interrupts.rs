#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

#[macro_use]
extern crate lazy_static;

use core::cell::RefCell;

use cortex_m_rt::entry;


use stm32f4::stm32f407::{self, Peripherals};
use stm32f4::stm32f407::interrupt;

use hartex_rust::events;
use hartex_rust::util::generate_task_mask;

use hartex_rust::tasks::*;
use hartex_rust::resources;

use hartex_rust::types::*;
use hartex_rust::spawn;
use cortex_m::peripheral::NVIC;

struct AppState {
    peripherals: Resource<RefCell<Peripherals>>,
    event_led1: EventId,
    event_led2: EventId,
}

lazy_static! {
    static ref globals: AppState = AppState {
        peripherals: resources::new(RefCell::new(Peripherals::take().unwrap()), generate_task_mask(&[1,2])).unwrap(),
        event_led1: events::new_OnOff(true).unwrap(),
        event_led2: events::new_OnOff(true).unwrap(),
    };
}

fn peripherals_init(peripherals: &mut Peripherals) {
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
    globals.peripherals.acquire(|peripherals| {
        let peripherals = &mut peripherals.borrow_mut();
        peripherals.EXTI.pr.write(|w| w.pr3().set_bit());
    });
    events::enable_event(globals.event_led1);
}

#[interrupt]
fn EXTI4() {
    globals.peripherals.acquire(|peripherals| {
        let peripherals = &mut peripherals.borrow_mut();
        peripherals.EXTI.pr.write(|w| w.pr4().set_bit());
    });
    events::enable_event(globals.event_led2);
}

#[entry]
fn main() -> ! {
    let peripherals = resources::init_peripherals().unwrap();

    globals.peripherals.acquire(|peripherals| {
        let peripherals = &mut peripherals.borrow_mut();
        peripherals_init(peripherals);
    });

    events::set_tasks(globals.event_led1, generate_task_mask(&[task1]));

    events::set_tasks(globals.event_led2, generate_task_mask(&[task2]));

    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];

    spawn!(task1, 1, stack1, params, globals, {
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
    spawn!(task2, 2, stack2, params, globals, {
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