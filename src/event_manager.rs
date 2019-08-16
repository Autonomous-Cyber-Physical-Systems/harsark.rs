use crate::config::{EVENT_INDEX_TABLE_COUNT, EVENT_NO};
use crate::task_manager::release;
use crate::{messaging::*, semaphores::*};
use cortex_m::interrupt::free as execute_critical;

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
        for i in 0..self.curr {
            dispatch_event(unsafe { &mut EVENT_TABLE[i] });
        }
    }
}

pub fn sweep_event_table(event_table: &EventIndexTable) {
    execute_critical(|_| unsafe {
        event_table.sweep();
    });
}

pub fn dispatch_event(event: &mut Event) {
    if event.is_enabled {
        event.counter -= 1;
        if event.counter == 0 {
            if event.event_type == EventType::FreeRunning {
                event.counter = event.threshold;
            } else {
                disable_event(event);
            }
            execute_opcode(event);
        }
    }
}

fn execute_opcode(event: &mut Event) {
    let opcode_signal = 1;
    let opcode_send_msg = 1 << 1;
    let opcode_release = 1 << 2;
    let opcode_enable_event = 1 << 3;

    if event.opcode & opcode_signal == opcode_signal {
        signal_and_release(event.semaphores, &event.tasks);
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
