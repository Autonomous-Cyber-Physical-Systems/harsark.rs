use crate::config::{EVENT_NO, EVENT_INDEX_TABLE_COUNT};

enum EventType {
    FreeRunning,
    OnOFF,
}

pub enum EventTimeType {
    MSec,
    Sec,
    Min,
    Hour,
}
struct Event {
    is_enabled: bool,
    event_type: EventType,
    threshold: u8,
    counter: u8,
    opcode: u8,
    semaphores: u32,
    tasks: u32,
    msg_index: u8,
    next_event: u32,
}

static mut EVENT_TABLE:[Event; EVENT_NO] = [Event; EVENT_NO];

static mut MS_EVENT_TABLE: [u8; EVENT_INDEX_TABLE_COUNT] = [0; EVENT_INDEX_TABLE_COUNT];
static mut SEC_EVENT_TABLE: [u8; EVENT_INDEX_TABLE_COUNT] = [0; EVENT_INDEX_TABLE_COUNT];
static mut MIN_EVENT_TABLE: [u8; EVENT_INDEX_TABLE_COUNT] = [0; EVENT_INDEX_TABLE_COUNT];
static mut HR_EVENT_TABLE: [u8; EVENT_INDEX_TABLE_COUNT] = [0; EVENT_INDEX_TABLE_COUNT];

pub fn sweep_event_table(event_type_type: EventTimeType) {}
pub fn dispatch_event(event_descriptor: usize) {}
pub fn enable_event(event_descriptor: usize) {}
pub fn disable_event(event_descriptor: usize) {}
fn execute_opcode(event_descriptor: usize) {}
fn enable_next(event_descriptor: usize) {}