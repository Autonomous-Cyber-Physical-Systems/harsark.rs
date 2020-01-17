//! # Event Manager
//!
//! Defines Data-structures to manage events.

use crate::config::EVENT_COUNT;
use crate::kernel::task_management::release;
use crate::system::types::{BooleanVector, EventId};
use crate::utils::errors::KernelError;

/// Event Descriptor
#[derive(Clone, Copy)]
pub struct Event
{
    /// Whether this event is currently enabled or not.
    is_enabled: bool,
    /// This is the frequency (of time unit in which it belongs to) in which the Event should run.
    threshold: u8,
    /// The current time elapsed. On reaching the value of the threshold, it is reset to zero, and the Event is dispatched.
    counter: u8,
    handler: fn() -> (),
}

impl Event {
    /// Takes the EventId and executes the corresponding event handler.
    pub fn execute_event(&mut self) {
        if self.is_enabled {
            if self.counter == 0 {
                self.counter = self.threshold;
                (self.handler)();
            } else {
                self.counter -= 1;
            }
        }
    }
}

/// Holds and Implements all Event management and dispatch methods.
pub struct EventTable 
{
    /// This array holds the Event descriptors of all events
    events: [Option<Event>; EVENT_COUNT],
    /// Points to the current empty slot in the `event_table`.
    curr: usize,
}

impl EventTable 
{
    /// Returns new instance of EventManager
    pub const fn new() -> Self {
        Self {
            events: [None; EVENT_COUNT],
            curr: 0,
        }
    }

    /// This function dispatches all events mentioned in the `EventIndexTable` corresponding to the `EventTableType`.
    pub fn sweep(&mut self) {
        for i in 0..self.curr {
            if let Some(ref mut event) = self.events[i] {
                event.execute_event();
            }
        }
    }

    /// Enables an Event.
    pub fn enable(&mut self, event_id: EventId) -> Result<(),KernelError> {
        let event = &mut self.events[event_id].as_mut().ok_or(KernelError::NotFound)?;
        event.is_enabled = true;
        Ok(())
    }

    /// Disables an Event.
    pub fn disable(&mut self, event_id: EventId) -> Result<(),KernelError> {
        let event = &mut self.events[event_id].as_mut().ok_or(KernelError::NotFound)?;
        event.is_enabled = false;
        Ok(())
    }

    /// Creates a new event.
    pub fn create(
        &mut self,
        is_enabled: bool,
        threshold: u8,
        handler: fn() -> ()
    ) -> Result<EventId, KernelError> {
        let id = self.curr;
        if id >= self.events.len() {
            return Err(KernelError::LimitExceeded);
        }
        self.events[id] = Some(Event {
            is_enabled,
            threshold,
            counter: 0,
            handler
        });
        self.curr += 1;
        return Ok(id);
    }
}
