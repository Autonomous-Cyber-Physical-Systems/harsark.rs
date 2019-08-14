enum EventType {
    FreeRunning,
    OnOFF
}
struct Event {
    is_enabled: bool,
    event_type: EventType,
    threshold: u8,
    counter: u8,
    opcode: u8,
}