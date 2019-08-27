use crate::config::{EVENT_INDEX_TABLE_COUNT, EVENT_NO};
use crate::task_manager::release;
use crate::{messaging::*, sync::*};
use cortex_m::interrupt::free as execute_critical;
use cortex_m_semihosting::hprintln;

#[derive(Clone, Copy, PartialEq)]
pub enum EventType {
    FreeRunning,
    OnOFF,
}

#[derive(Clone, Copy)]
pub struct Event {
    is_enabled: bool,
    event_type: EventType,
    threshold: u8,
    counter: u8,
    opcode: u8,
    semaphores: usize,
    tasks: u32,
    msg_index: usize,
    next_event: usize,
}

pub struct EventIndexTable {
    table: [u8; EVENT_INDEX_TABLE_COUNT],
    curr: usize,
}

static mut EVENT_TABLE: [Event; EVENT_NO] = [Event {
    is_enabled: false,
    event_type: EventType::FreeRunning,
    threshold: 0,
    counter: 0,
    opcode: 0,
    semaphores: 0,
    tasks: 0,
    msg_index: 0,
    next_event: 0,
}; EVENT_NO];

pub static mut MS_EVENT_TABLE: EventIndexTable = EventIndexTable {
    table: [0; EVENT_INDEX_TABLE_COUNT],
    curr: 0,
};
pub static mut SEC_EVENT_TABLE: EventIndexTable = EventIndexTable {
    table: [0; EVENT_INDEX_TABLE_COUNT],
    curr: 0,
};
pub static mut MIN_EVENT_TABLE: EventIndexTable = EventIndexTable {
    table: [0; EVENT_INDEX_TABLE_COUNT],
    curr: 0,
};
pub static mut HR_EVENT_TABLE: EventIndexTable = EventIndexTable {
    table: [0; EVENT_INDEX_TABLE_COUNT],
    curr: 0,
};

impl EventIndexTable {
    pub fn sweep(&self) {
        execute_critical(|_| {
            for i in 0..self.curr {
                execute_event(unsafe { &mut EVENT_TABLE[i] });
            }
        })
    }
}

pub fn sweep_event_table(event_table: &EventIndexTable) {
    execute_critical(|_| unsafe {
        event_table.sweep();
    });
}

pub fn execute_event(event: &mut Event) {
    execute_critical(|_| {
        if event.is_enabled {
            if event.counter == 0 {
                if event.event_type == EventType::FreeRunning {
                    event.counter = event.threshold;
                } else {
                    disable_event(event);
                }
                execute_opcode(event);
            }
            event.counter -= 1;
        }
    });
}

fn execute_opcode(event: &mut Event) {
    execute_critical(|_| {
        let opcode_signal = 1;
        let opcode_send_msg = 1 << 1;
        let opcode_release = 1 << 2;
        let opcode_enable_event = 1 << 3;

        if event.opcode & opcode_signal == opcode_signal {
            //            signal_and_release(event.semaphores, &event.tasks);
        }
        if event.opcode & opcode_send_msg == opcode_send_msg {
            broadcast(event.msg_index);
        }
        if event.opcode & opcode_release == opcode_release {
            release(&event.tasks);
        }
        if event.opcode & opcode_enable_event == opcode_enable_event {
            enable_next(event);
        }
    })
}

pub fn enable_event(event: &mut Event) {
    execute_critical(|_| unsafe {
        event.is_enabled = true;
    });
}

pub fn disable_event(event: &mut Event) {
    execute_critical(|_| unsafe {
        event.is_enabled = false;
    });
}

pub fn enable_next(event: &mut Event) {
    execute_critical(|_| unsafe {
        EVENT_TABLE[event.next_event].is_enabled = true;
    });
}

pub fn dispatch_event(event_descriptor: usize) {
    execute_critical(|_| {
        let mut event = unsafe { &mut EVENT_TABLE[event_descriptor] };
        execute_event(&mut event);
    });
}

pub fn define_event(
    ind: usize,
    is_enabled: bool,
    event_type: EventType,
    threshold: u8,
    opcode: u8,
    semaphores: usize,
    tasks: u32,
    msg_index: usize,
    next_event: usize,
) {
    execute_critical(|_| {
        let mut event_table = unsafe { &mut EVENT_TABLE };
        event_table[ind].is_enabled = is_enabled;
        event_table[ind].event_type = event_type;
        event_table[ind].threshold = threshold;
        event_table[ind].opcode = opcode;
        event_table[ind].semaphores = semaphores;
        event_table[ind].tasks = tasks;
        event_table[ind].msg_index = msg_index;
        event_table[ind].next_event = next_event;
    });
}
