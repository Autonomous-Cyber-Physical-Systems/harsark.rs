use crate::config::{EVENT_COUNT, EVENT_INDEX_TABLE_COUNT};
use crate::config::{OPCODE_ENABLE_EVENT, OPCODE_RELEASE, OPCODE_SEND_MSG, OPCODE_SIGNAL};
use crate::system::types::{EventId, MessageId, SemaphoreId};
use crate::kernel::messages::broadcast;
use crate::kernel::tasks::release;
use crate::kernel::semaphores::signal_and_release;

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
    OnOff,
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
    event_table: [Option<Event>; EVENT_COUNT],
    curr: usize,
    ms_event_table: EventIndexTable,
    sec_event_table: EventIndexTable,
    min_event_table: EventIndexTable,
    hr_event_table: EventIndexTable,
    onoff_event_table: EventIndexTable,
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
            event_table: [None; EVENT_COUNT],
            curr: 0,
            ms_event_table: EventIndexTable::new(),
            sec_event_table: EventIndexTable::new(),
            min_event_table: EventIndexTable::new(),
            hr_event_table: EventIndexTable::new(),
            onoff_event_table: EventIndexTable::new(),
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
            EventTableType::OnOff => {
                self.onoff_event_table
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
        let event = &mut self.event_table[event_id].as_mut().unwrap();
        if event.is_enabled {
            if event.counter == 0 {
                if event.event_type == EventType::FreeRunning {
                    event.counter = event.threshold;
                } else {
                    self.disable_event(event_id);
                }
                self.execute_opcode(event_id);
            } else {
                event.counter -= 1;
            }
        }
    }

    fn execute_opcode(&mut self, event_id: EventId) {
        let event = self.event_table[event_id].as_ref().unwrap();

        if event.opcode & OPCODE_SIGNAL == OPCODE_SIGNAL {
            signal_and_release(event.semaphore, event.tasks);
        }
        if event.opcode & OPCODE_SEND_MSG == OPCODE_SEND_MSG {
            broadcast(event.msg_index);
        }
        if event.opcode & OPCODE_RELEASE == OPCODE_RELEASE {
            release(event.tasks);
        }
        if event.opcode & OPCODE_ENABLE_EVENT == OPCODE_ENABLE_EVENT {
            self.enable_event(event_id);
        }
    }

    pub fn enable_event(&mut self, event_id: EventId) {
        let event = &mut self.event_table[event_id].as_mut().unwrap();
        event.is_enabled = true;
    }

    pub fn disable_event(&mut self, event_id: EventId) {
        let event = &mut self.event_table[event_id].as_mut().unwrap();
        event.is_enabled = false;
    }

    pub fn create(
        &mut self,
        is_enabled: bool,
        event_type: EventType,
        threshold: u8,
        event_counter_type: EventTableType,
    ) -> EventId {
        let id = self.curr;
        self.event_table[id] = Some(Event {
            is_enabled,
            event_type,
            threshold,
            counter: 0,
            opcode: 0,
            semaphore: 0,
            tasks: 0,
            msg_index: 0,
            next_event: 0,
        });
        match event_counter_type {
            EventTableType::Hour => self.hr_event_table.add(self.curr),
            EventTableType::MilliSec => self.ms_event_table.add(self.curr),
            EventTableType::Min => self.min_event_table.add(self.curr),
            EventTableType::Sec => self.sec_event_table.add(self.curr),
            EventTableType::OnOff => self.onoff_event_table.add(self.curr),
        };
        self.curr += 1;
        return id;
    }

    pub fn set_semaphore(&mut self, event_id: EventId, sem: SemaphoreId, tasks_mask: u32) {
        let event = &mut self.event_table[event_id].as_mut().unwrap();
        event.opcode |= OPCODE_SIGNAL;
        event.semaphore = sem;
        event.tasks |= tasks_mask;
    }

    pub fn set_tasks(&mut self, event_id: EventId, tasks_mask: u32) {
        let event = &mut self.event_table[event_id].as_mut().unwrap();
        event.opcode |= OPCODE_RELEASE;
        event.tasks = tasks_mask;
    }

    pub fn set_message(&mut self, event_id: EventId, msg_id: usize) {
        let event = &mut self.event_table[event_id].as_mut().unwrap();
        event.opcode |= OPCODE_SEND_MSG;
        event.msg_index = msg_id;
    }

    pub fn set_next_event(&mut self, event_id: EventId, next: EventId) {
        let event = &mut self.event_table[event_id].as_mut().unwrap();
        event.opcode |= OPCODE_ENABLE_EVENT;
        event.next_event = next;
    }
}
