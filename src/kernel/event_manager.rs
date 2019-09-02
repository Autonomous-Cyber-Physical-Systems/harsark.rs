use crate::config::{EVENT_INDEX_TABLE_COUNT, EVENT_NO};
use crate::helper::generate_task_mask;
use crate::kernel::semaphores::SemaphoreId;
use crate::kernel::task_manager::{release, TaskId};
use crate::{messaging::*, sync::*};
use cortex_m::interrupt::free as execute_critical;
use cortex_m_semihosting::hprintln;

pub type EventId = usize;

#[derive(Clone, Copy, PartialEq)]
pub enum EventType {
    FreeRunning,
    OnOFF,
}

#[derive(Clone, Copy, PartialEq)]
pub enum EventTableType {
    MilliSec,
    Sec,
    Min,
    Hour,
}

#[derive(Clone, Copy)]
pub struct Event {
    is_enabled: bool,
    event_type: EventType,
    threshold: u8,
    counter: u8,
    opcode: u8,
    semaphores: u32,
    tasks: u32,
    msg_index: usize,
    next_event: usize,
}

pub struct EventIndexTable {
    table: [usize; EVENT_INDEX_TABLE_COUNT],
    curr: usize,
}

pub struct EventManager {
    event_table: [Event; EVENT_NO],
    curr: usize,
    ms_event_table: EventIndexTable,
    sec_event_table: EventIndexTable,
    min_event_table: EventIndexTable,
    hr_event_table: EventIndexTable,
}

impl EventIndexTable {
    pub const fn new() -> Self {
        Self {
            table: [0; EVENT_INDEX_TABLE_COUNT],
            curr: 0,
        }
    }

    pub fn add(&mut self, id: EventId) {
        self.table[self.curr] = id;
        self.curr += 1;
    }
}

impl EventManager {
    pub const fn new() -> Self {
        Self {
            event_table: [Event {
                is_enabled: false,
                event_type: EventType::FreeRunning,
                threshold: 0,
                counter: 0,
                opcode: 0,
                semaphores: 0,
                tasks: 0,
                msg_index: 0,
                next_event: 0,
            }; EVENT_NO],
            curr: 0,
            ms_event_table: EventIndexTable::new(),
            sec_event_table: EventIndexTable::new(),
            min_event_table: EventIndexTable::new(),
            hr_event_table: EventIndexTable::new(),
        }
    }

    pub fn sweep(&mut self, event_type: EventTableType) {
        match event_type {
            EventTableType::MilliSec => {
                self.ms_event_table
                    .table
                    .clone()
                    .iter()
                    .for_each(|event_id| {
                        self.execute_event(*event_id);
                    });
            }
            EventTableType::Sec => {
                self.sec_event_table
                    .table
                    .clone()
                    .iter()
                    .for_each(|event_id| {
                        self.execute_event(*event_id);
                    });
            }
            EventTableType::Min => {
                self.min_event_table
                    .table
                    .clone()
                    .iter()
                    .for_each(|event_id| {
                        self.execute_event(*event_id);
                    });
            }
            EventTableType::Hour => {
                self.hr_event_table
                    .table
                    .clone()
                    .iter()
                    .for_each(|event_id| {
                        self.execute_event(*event_id);
                    });
            }
        }
    }

    pub fn execute_event(&mut self, event_id: EventId) {
        let event = &self.event_table[event_id];
        if event.is_enabled {
            if event.counter == 0 {
                if event.event_type == EventType::FreeRunning {
                    self.event_table[event_id].counter = event.threshold;
                } else {
                    self.disable_event(event_id);
                }
                self.execute_opcode(event_id);
            }
            self.event_table[event_id].counter -= 1;
        }
    }

    fn execute_opcode(&mut self, event_id: EventId) {
        let event = self.event_table[event_id];
        let opcode_signal = 1;
        let opcode_send_msg = 1 << 1;
        let opcode_release = 1 << 2;
        let opcode_enable_event = 1 << 3;

        if event.opcode & opcode_signal == opcode_signal {
            //                sem_post(event.semaphores, &event.tasks);
        }
        if event.opcode & opcode_send_msg == opcode_send_msg {
            broadcast(event.msg_index);
        }
        if event.opcode & opcode_release == opcode_release {
            release(&event.tasks);
        }
        if event.opcode & opcode_enable_event == opcode_enable_event {
            self.enable_next(event_id);
        }
    }

    pub fn enable_event(&mut self, event_id: EventId) {
        let mut event = self.event_table[event_id];
        event.is_enabled = true;
    }

    pub fn disable_event(&mut self, event_id: EventId) {
        let mut event = self.event_table[event_id];
        event.is_enabled = false;
    }

    pub fn enable_next(&mut self, event_id: EventId) {
        let mut event = self.event_table[event_id];
        self.event_table[event.next_event].is_enabled = true;
    }

    pub fn create(
        &mut self,
        is_enabled: bool,
        event_type: EventType,
        threshold: u8,
        opcode: u8,
        semaphores: &[u32],
        tasks: &[u32],
        msg_index: usize,
        next_event: usize,
    ) {
        self.event_table[self.curr] = Event {
            is_enabled,
            event_type,
            threshold,
            counter: 0,
            opcode,
            semaphores: generate_task_mask(semaphores),
            tasks: generate_task_mask(tasks),
            msg_index,
            next_event,
        };
    }
}
