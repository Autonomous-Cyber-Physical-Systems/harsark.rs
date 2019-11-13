#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::events;
use hartex_rust::messages;
use hartex_rust::resources;
use hartex_rust::semaphores;
use hartex_rust::spawn;
use hartex_rust::tasks::*;
use hartex_rust::types::*;
use hartex_rust::util::generate_task_mask;

struct AppState {
    sem2: SemaphoreId,
    msg1: Message<[u32; 2]>,
}

#[entry]
fn main() -> ! {
    let peripherals = resources::init_peripherals().unwrap();

    let app_inst = AppState {
        sem2: semaphores::new(generate_task_mask(&[task2])).unwrap(),
        msg1: messages::new(
            generate_task_mask(&[task3]),
            generate_task_mask(&[task3]),
            [9, 10],
        )
        .unwrap(),
    };

    /*
    Creates a FreeRunning Event that occurs once in every second.
    The event releases task1 when its counter expires.
    */
    let e1 = events::new_FreeRunning(true, 1, EventTableType::Sec).unwrap();
    events::set_tasks(e1, generate_task_mask(&[task1]));

    /*
    Creates a FreeRunning Event that occurs once in every 2 seconds.
    The event signals semaphore sem2 when its counter expires.
    */
    let e2 = events::new_FreeRunning(true, 2, EventTableType::Sec).unwrap();
    events::set_semaphore(e2, app_inst.sem2, generate_task_mask(&[task1, task2]));

    /*
    Creates an OnOff Event.
    The event broadcasts message msg1 whenever it is dispatched.
    */
    let e3 = events::new_OnOff(false).unwrap();
    events::set_message(e3, app_inst.msg1.get_id());

    /*
    Creates a FreeRunning Event that occurs once in every 3 seconds.
    The event dispatches event3 when its counter expires.
    */
    let e4 = events::new_FreeRunning(true, 3, EventTableType::Sec).unwrap();
    events::set_next_event(e4, e3);

    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];
    static mut stack3: [u32; 300] = [0; 300];

    spawn!(task1, 1, stack1, params, app_inst, {
        hprintln!("TASK 1: Enter");
        if let Ok(true) = semaphores::test_and_reset(params.sem2) {
            hprintln!("TASK 1: sem2 enabled");
        }
        hprintln!("TASK 1: End");
    });
    spawn!(task2, 2, stack2, params, app_inst, {
        hprintln!("TASK 2: Enter");
        if let Ok(true) = semaphores::test_and_reset(params.sem2) {
            hprintln!("TASK 2: sem2 enabled");
        }
        hprintln!("TASK 2: End");
    });
    spawn!(task3, 3, stack3, params, app_inst, {
        hprintln!("TASK 3: Enter");
        if let Some(msg) = params.msg1.receive() {
            hprintln!("TASK 3: msg received : {:?}", msg);
        }
        hprintln!("TASK 3: End");
    });

    init(true);
    release(task1);
    start_kernel(
        unsafe { &mut peripherals.access().unwrap().borrow_mut() },
        150_000,
    );
    loop {}
}
