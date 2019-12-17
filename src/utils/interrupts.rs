use crate::kernel::events::sweep_event_table;
use crate::kernel::tasks::{is_preemptive, schedule};
use crate::kernel::time::{tick};
use crate::system::event_manager::EventTableType;
use crate::system::time_manager::TickType;
use crate::utils::arch::switch_to_user_level;
use crate::kernel::tasks::{os_curr_task_id, os_next_task_id, scheduler};
use cortex_m::interrupt::free as execute_critical;
use cortex_m_rt::exception;

// SysTick Exception handler
#[exception]
fn SysTick() {
    // hprintln!("{:?}", cortex_m::register::control::read().npriv());
    if is_preemptive() {
        schedule();
    }

    sweep_event_table(EventTableType::OnOff);
    sweep_event_table(EventTableType::MilliSec);

    match tick() {
        TickType::Hour => {
            sweep_event_table(EventTableType::Sec);
            sweep_event_table(EventTableType::Min);
            sweep_event_table(EventTableType::Hour);
        }
        TickType::Min => {
            sweep_event_table(EventTableType::Sec);
            sweep_event_table(EventTableType::Min);
        }
        TickType::Sec => {
            sweep_event_table(EventTableType::Sec);
        }
        _ => {}
    }
}

#[exception]
fn SVCall() {
    schedule();
}

#[exception]
fn PendSV() {
    execute_critical(|cs_token| {

        let curr_tid: usize = *os_curr_task_id.borrow(cs_token).borrow();
        let next_tid: usize = *os_next_task_id.borrow(cs_token).borrow();
        let scheduler_inst = scheduler.borrow(cs_token).borrow_mut();
        let next_task = scheduler_inst.task_control_blocks[next_tid].as_ref().unwrap();

		if scheduler_inst.started {
			let curr_task = scheduler_inst.task_control_blocks[curr_tid].as_ref().unwrap();
			curr_task.save_context();
       	}

		next_task.load_context();

    });
    switch_to_user_level();
}
