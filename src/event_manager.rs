enum EventType {
    FreeRunning,
    OnOFF,
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
