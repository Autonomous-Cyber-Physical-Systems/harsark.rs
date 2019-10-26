use crate::config::{EVENT_INDEX_TABLE_COUNT, EVENT_NO};
use crate::config::{OPCODE_ENABLE_EVENT, OPCODE_RELEASE, OPCODE_SEND_MSG, OPCODE_SIGNAL};
use crate::internals::types::{EventId, MessageId, SemaphoreId};
use crate::message::broadcast;
use crate::process::release;
use crate::sync::sem_set;
use cortex_m::interrupt::free as execute_critical;
use cortex_m_semihosting::hprintln;

#[derive(Clone, Copy, PartialEq)]
pub enum EventType {
    FreeRunning,
    OnOff,
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
    semaphore: SemaphoreId,
    tasks: u32,
    msg_index: MessageId,
    next_event: EventId,
}

pub struct EventIndexTable {
    table: [Option<usize>; EVENT_INDEX_TABLE_COUNT],
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
            table: [None; EVENT_INDEX_TABLE_COUNT],
            curr: 0,
        }
    }

    pub fn add(&mut self, id: EventId) {
        self.table[self.curr] = Some(id);
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
                semaphore: 0,
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
                        if let Some(event_id) = event_id {
                            self.execute_event(*event_id);
                        }
                    });
            }
            EventTableType::Sec => {
                self.sec_event_table
                    .table
                    .clone()
                    .iter()
                    .for_each(|event_id| {
                        if let Some(event_id) = event_id {
                            self.execute_event(*event_id);
                        }
                    });
            }
            EventTableType::Min => {
                self.min_event_table
                    .table
                    .clone()
                    .iter()
                    .for_each(|event_id| {
                        if let Some(event_id) = event_id {
                            self.execute_event(*event_id);
                        }
                    });
            }
            EventTableType::Hour => {
                self.hr_event_table
                    .table
                    .clone()
                    .iter()
                    .for_each(|event_id| {
                        if let Some(event_id) = event_id {
                            self.execute_event(*event_id);
                        }
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
            } else {
                self.event_table[event_id].counter -= 1;
            }
        }
    }

    fn execute_opcode(&mut self, event_id: EventId) {
        let event = self.event_table[event_id];

        if event.opcode & OPCODE_SIGNAL == OPCODE_SIGNAL {
            sem_set(event.semaphore, event.tasks);
        }
        if event.opcode & OPCODE_SEND_MSG == OPCODE_SEND_MSG {
            broadcast(event.msg_index);
        }
        if event.opcode & OPCODE_RELEASE == OPCODE_RELEASE {
            release(event.tasks);
        }
        if event.opcode & OPCODE_ENABLE_EVENT == OPCODE_ENABLE_EVENT {
            self.enable_next(event_id);
        }
    }

    pub fn enable_event(&mut self, event_id: EventId) {
        let mut event = self.event_table[event_id];
        event.is_enabled = true;
    }

    pub fn disable_event(&mut self, event_id: EventId) {
        self.event_table[event_id].is_enabled = false;
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
        event_counter_type: EventTableType,
    ) -> EventId {
        let id = self.curr;
        self.event_table[id] = Event {
            is_enabled,
            event_type,
            threshold,
            counter: 0,
            opcode: 0,
            semaphore: 0,
            tasks: 0,
            msg_index: 0,
            next_event: 0,
        };
        match event_counter_type {
            EventTableType::Hour => self.hr_event_table.add(self.curr),
            EventTableType::MilliSec => self.ms_event_table.add(self.curr),
            EventTableType::Min => self.min_event_table.add(self.curr),
            EventTableType::Sec => self.sec_event_table.add(self.curr),
        };
        self.curr += 1;
        return id;
    }

    pub fn set_semaphore(&mut self, event_id: EventId, sem: SemaphoreId, tasks_mask: u32) {
        self.event_table[event_id].opcode |= OPCODE_SIGNAL;
        self.event_table[event_id].semaphore = sem;
        self.event_table[event_id].tasks |= tasks_mask;
    }

    pub fn set_tasks(&mut self, event_id: EventId, tasks_mask: u32) {
        self.event_table[event_id].opcode |= OPCODE_RELEASE;
        self.event_table[event_id].tasks = tasks_mask;
    }

    pub fn set_msg(&mut self, event_id: EventId, msg_id: usize) {
        self.event_table[event_id].opcode |= OPCODE_SEND_MSG;
        self.event_table[event_id].msg_index = msg_id;
    }

    pub fn set_next_event(&mut self, event_id: EventId, next: EventId) {
        self.event_table[event_id].opcode |= OPCODE_ENABLE_EVENT;
        self.event_table[event_id].next_event = next;
    }
}
