#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::helper::generate_task_mask;
use hartex_rust::message::{self, Message};
use hartex_rust::process::*;
use hartex_rust::resource::{self, Resource};
use hartex_rust::sync;
use hartex_rust::types::*;
use hartex_rust::spawn;

struct app {
    sem3: SemaphoreId,
    msg1: Message<[u32; 2]>,
}

#[entry]
fn main() -> ! {
    let peripherals = resource::init_peripherals().unwrap();

    let app_inst = app {
        sem3: sync::create(generate_task_mask(&[3])).unwrap(),
        msg1: message::create(
            generate_task_mask(&[2]),
            generate_task_mask(&[2, 3]),
            [9, 10],
        )
        .unwrap(),
    };

    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];
    static mut stack3: [u32; 300] = [0; 300];

    spawn!(thread1, 1, stack1, params, app_inst, {
        hprintln!("TASK 1: Enter");
        params.msg1.broadcast(Some([4, 5]));
        sync::sem_set(params.sem3, 0);
        hprintln!("TASK 1: END");
    });
    spawn!(thread2, 2, stack2, params, app_inst, {
        hprintln!("TASK 2: Enter");
        if let Some(msg) = params.msg1.receive() {
            hprintln!("TASK 2: msg received : {:?}", msg);
        }
        hprintln!("TASK 2: END");
    });
    spawn!(thread3, 3, stack3, params, app_inst, {
        hprintln!("TASK 3: Enter");
        if let Some(msg) = params.msg1.receive() {
            hprintln!("TASK 3: msg received : {:?}", msg);
        }
        hprintln!("TASK 3: END");
    });

    init(true);
    release(generate_task_mask(&[1]));
    start_kernel(unsafe{&mut peripherals.access().unwrap().borrow_mut()}, 150_000);loop {}
}
