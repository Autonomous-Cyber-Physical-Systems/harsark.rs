use crate::system::scheduler::{BooleanVector,TaskId};
use crate::system::event::EventId;
use crate::config::MAX_LOGS;
use core::fmt;

pub type Logs = [Option<LogEvent>; MAX_LOGS];

#[derive(Clone, Copy)]
pub enum LogEventType {
    ReleaseTasks(BooleanVector),
    BlockTasks(BooleanVector),
    UnblockTasks(BooleanVector),
    TaskExit(BooleanVector),
    ResourceLock(TaskId,bool),
    ResourceUnlock(TaskId),
    MessageBroadcast(BooleanVector,BooleanVector),
    MessageRecieve(TaskId),
    SemaphoreSignal(BooleanVector,BooleanVector),
    SemaphoreReset(TaskId),
    TimerEvent(EventId),
}

#[derive(Clone, Copy)]
pub struct LogEvent {
    event_type: LogEventType,
    timestamp: u32
}

impl LogEvent {
    pub fn new(event_type: LogEventType, timestamp: u32) -> Self {
        Self {
            event_type,
            timestamp
        }
    }
}

pub struct Logger {
    logs: Logs,
    start: usize,
    end: usize,
}
// use a circular queue instead of this crap.
// ensure the handler is not None in start_kernel.
impl Logger {
    pub const fn new() -> Self {
        Self {
            logs: [None; MAX_LOGS],
            start: 0,
            end: 0,
        }
    }
    pub fn push(&mut self, event: LogEvent) {
        self.logs[self.end] = Some(event);
        self.end += 1;
        if self.start == self.end {
            self.start += 1;
        }
    }
    pub fn clear(&mut self) {
        for val in self.logs.iter_mut() {
            *val = None;
        }
        self.start = 0;
        self.end = 0;
    }
    pub fn pop(&mut self) -> Option<LogEvent> {
        if self.start == self.end {
            return None;
        }
        let pos = self.start;
        let val = self.logs[pos];
        self.logs[pos] = None;
        self.start += 1;
        return val;
    }
}

impl fmt::Debug for LogEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LogEventType::ReleaseTasks(tasks_mask) => write!(f, "Tasks Released: {}", tasks_mask),
            LogEventType::BlockTasks(tasks_mask) => write!(f, "Exists"),
            LogEventType::UnblockTasks(tasks_mask) => write!(f, "Exists"),
            LogEventType::TaskExit(tasks_mask) => write!(f, "Exists"),
            LogEventType::ResourceLock(ceiling,bool) => write!(f, "Exists"),
            LogEventType::ResourceUnlock(ceiling) => write!(f, "Exists"),
            LogEventType::MessageBroadcast(tasks_released,recieved) => write!(f, "Exists"),
            LogEventType::MessageRecieve(task_id) => write!(f, "Exists"),
            LogEventType::SemaphoreSignal(tasks_released,tasks_notified) => write!(f, "Exists"),
            LogEventType::SemaphoreReset(task_id) => write!(f, "Exists"),
            LogEventType::TimerEvent(EventId) => write!(f, "Exists"),
        }
    }
}
