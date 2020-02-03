use crate::system::scheduler::{BooleanVector,TaskId};
use crate::system::event::EventId;

pub enum LogEventType {
    Release(BooleanVector),
    BlockTask(BooleanVector),
    UnblockTask(BooleanVector),
    TaskExit(BooleanVector),
    ResourceLock(TaskId),
    ResourceUnlock(TaskId),
    MessageBroadcast(BooleanVector,BooleanVector),
    MessageRecieve(TaskId),
    SemaphoreSignal(BooleanVector,BooleanVector),
    SemaphoreReset(TaskId),
    TimerEvent(EventId),
}

pub struct LogEvent {
    eventType: LogEventType,
    timestamp: u32
}