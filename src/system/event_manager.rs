//! # Event Manager
//!
//! Defines Data-structures to manage events.

use crate::config::{EVENT_COUNT, EVENT_INDEX_TABLE_COUNT};
use crate::config::{OPCODE_ENABLE_EVENT, OPCODE_RELEASE, OPCODE_SEND_MSG, OPCODE_SIGNAL};
use crate::kernel::messages::broadcast;
use crate::kernel::semaphores::signal_and_release;
use crate::kernel::tasks::release;
use crate::system::types::{BooleanVector, EventId, MessageId, SemaphoreId};
use crate::utils::errors::KernelError;

#[derive(Clone, Copy, PartialEq)]
pub enum EventType {
    /// It represents events that repeatedly occur after a particular time threshold.
    FreeRunning,
    ///  It represents events that are dispatched once and are then disabled.
    /// They are executed later only if Some task enables the Event explicitly.
    OnOff,
}

/// Events are executed at multiples of certain time units. For example, an event can be dispatched
/// once every 40 milliseconds, or 50 seconds, etc. This enum represents each such time units. Note
/// that this includes OnOff also as OnOff events do not belong to the category of time units; hence,
/// it is given as a separate field.
#[derive(Clone, Copy, PartialEq)]
pub enum EventTableType {
    MilliSec,
    Sec,
    Min,
    Hour,
    OnOff,
}

/// Event Descriptor
#[derive(Clone, Copy)]
pub struct Event {
    /// Whether this event is currently enabled or not.
    is_enabled: bool,
    /// Is the event OnOff or FreeRunning.
    event_type: EventType,
    /// This is the frequency (of time unit in which it belongs to) in which the Event should run.
    threshold: u8,
    /// The current time elapsed. On reaching the value of the threshold, it is reset to zero, and the Event is dispatched.
    counter: u8,
    /// A 4-bit code that corresponds to what are the operations that this event corresponds to.
    opcode: u8,
    /// The SemaphoreId of the Semaphore that has to be signaled when the event is dispatched.
    semaphore: Option<SemaphoreId>,
    /// The `task_mask` of the tasks that have to be released or signaled (in case of semaphore event) when the event is dispatched.
    tasks: Option<BooleanVector>,
    /// The MessageId of the message corresponding to the index.
    msg_index: Option<MessageId>,
    /// The EventId of the next event that has to be triggered by this event.
    next_event: Option<EventId>,
}


/// An EventIndexTable is created for each of the elements of EventTableType.
pub struct EventIndexTable {
    /// Holds the list of EventIds of events that belong to the particular EventTableType
    /// this EventIndexTable belongs to.
    table: [Option<usize>; EVENT_INDEX_TABLE_COUNT],
    /// It is just an index to the next free location in the table.
    curr: usize,
}

/// Holds and Implements all Event management and dispatch methods.
pub struct EventManager {
    /// This array holds the Event descriptors of all events
    event_table: [Option<Event>; EVENT_COUNT],
    /// Points to the current empty slot in the `event_table`.
    curr: usize,
    /// An instance of `EventIndexTable` which holds list of EventIds of type `EventTableType::MilliSec`.
    ms_event_table: EventIndexTable,
    /// An instance of `EventIndexTable` which holds list of EventIds of type `EventTableType::Sec`.
    sec_event_table: EventIndexTable,
    /// An instance of `EventIndexTable` which holds list of EventIds of type `EventTableType::Min`.
    min_event_table: EventIndexTable,
    /// An instance of `EventIndexTable` which holds list of EventIds of type `EventTableType::Hour`.
    hr_event_table: EventIndexTable,
    /// An instance of `EventIndexTable` which holds list of EventIds of type `EventTableType::OnOff`.
    onoff_event_table: EventIndexTable,
}

impl EventIndexTable {
    /// Return an new instance of EventIndexTable.
    pub const fn new() -> Self {
        Self {
            table: [None; EVENT_INDEX_TABLE_COUNT],
            curr: 0,
        }
    }

    /// Adds `id` to the EventIndexTable
    pub fn add(&mut self, id: EventId) -> Result<(), KernelError> {
        if self.curr >= EVENT_INDEX_TABLE_COUNT {
            return Err(KernelError::LimitExceeded);
        }
        self.table[self.curr] = Some(id);
        self.curr += 1;
        Ok(())
    }
}

impl EventManager {
    /// Returns new instance of EventManager
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

    /// This function dispatches all events mentioned in the `EventIndexTable` corresponding to the `EventTableType`.
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

    /// Takes the EventId and executes the corresponding event handler.
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

    /// This method executes all the operations corresponding to the opcode of the task.
    fn execute_opcode(&mut self, event_id: EventId) {
        let event = self.event_table[event_id].as_ref().unwrap();

        if event.opcode & OPCODE_SIGNAL == OPCODE_SIGNAL {
            signal_and_release(event.semaphore.unwrap(), event.tasks.unwrap());
        }
        if event.opcode & OPCODE_SEND_MSG == OPCODE_SEND_MSG {
            broadcast(event.msg_index.unwrap());
        }
        if event.opcode & OPCODE_RELEASE == OPCODE_RELEASE {
            release(event.tasks.unwrap());
        }
        if event.opcode & OPCODE_ENABLE_EVENT == OPCODE_ENABLE_EVENT {
            self.enable_event(event.next_event.unwrap());
        }
    }

    /// Enables an Event.
    pub fn enable_event(&mut self, event_id: EventId) {
        let event = &mut self.event_table[event_id].as_mut().unwrap();
        event.is_enabled = true;
    }

    /// Disables an Event.
    pub fn disable_event(&mut self, event_id: EventId) {
        let event = &mut self.event_table[event_id].as_mut().unwrap();
        event.is_enabled = false;
    }

    /// Creates a new event.
    pub fn create(
        &mut self,
        is_enabled: bool,
        event_type: EventType,
        threshold: u8,
        event_counter_type: EventTableType,
    ) -> Result<EventId, KernelError> {
        let id = self.curr;
        if id >= self.event_table.len() {
            return Err(KernelError::LimitExceeded);
        }
        self.event_table[id] = Some(Event {
            is_enabled,
            event_type,
            threshold,
            counter: 0,
            opcode: 0,
            semaphore: None,
            tasks: None,
            msg_index: None,
            next_event: None,
        });
        match event_counter_type {
            EventTableType::Hour => self.hr_event_table.add(self.curr),
            EventTableType::MilliSec => self.ms_event_table.add(self.curr),
            EventTableType::Min => self.min_event_table.add(self.curr),
            EventTableType::Sec => self.sec_event_table.add(self.curr),
            EventTableType::OnOff => self.onoff_event_table.add(self.curr),
        };
        self.curr += 1;
        return Ok(id);
    }

    /// Updates the opcode by setting the bit corresponding to Semaphore and stores the SemaphoreId and Tasks Boolean Vector.
    pub fn set_semaphore(
        &mut self,
        event_id: EventId,
        sem: SemaphoreId,
        tasks_mask: BooleanVector,
    ) -> Result<(), KernelError> {
        let event = &mut self.event_table[event_id].as_mut().unwrap();
        event.opcode |= OPCODE_SIGNAL;
        if event.semaphore.is_none() {
            event.semaphore.replace(sem);
        } else {
            return Err(KernelError::Exists);
        }
        if event.tasks.is_none() {
            event.tasks.replace(tasks_mask);
        } else {
            return Err(KernelError::Exists);
        }
        Ok(())
    }

    /// Updates the opcode by setting the bit corresponding to Release Tasks and stores the `tasks_mask` Boolean Vector.
    pub fn set_tasks(
        &mut self,
        event_id: EventId,
        tasks_mask: BooleanVector,
    ) -> Result<(), KernelError> {
        let event = &mut self.event_table[event_id].as_mut().unwrap();
        event.opcode |= OPCODE_RELEASE;
        if event.tasks.is_none() {
            event.tasks.replace(tasks_mask);
        } else {
            return Err(KernelError::Exists);
        }
        Ok(())
    }

    /// Updates the opcode by setting the bit corresponding to the Next event and stores the EventId.
    pub fn set_message(&mut self, event_id: EventId, msg_id: MessageId) -> Result<(), KernelError> {
        let event = &mut self.event_table[event_id].as_mut().unwrap();
        event.opcode |= OPCODE_SEND_MSG;
        if event.msg_index.is_none() {
            event.msg_index.replace(msg_id);
        } else {
            return Err(KernelError::Exists);
        }
        Ok(())
    }

    /// Updates the opcode by setting the bit corresponding to Message and stores the MessageId.
    pub fn set_next_event(&mut self, event_id: EventId, next: EventId) -> Result<(), KernelError> {
        let event = &mut self.event_table[event_id].as_mut().unwrap();
        event.opcode |= OPCODE_ENABLE_EVENT;
        if event.next_event.is_none() {
            event.next_event.replace(next);
        } else {
            return Err(KernelError::Exists);
        }
        Ok(())
    }
}
