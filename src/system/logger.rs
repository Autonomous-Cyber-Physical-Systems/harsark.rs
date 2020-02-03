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

#[derive(Clone, Copy, Debug)]
pub struct LogEvent {
    pub event_type: LogEventType,
    pub timestamp: u32
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
    pub release_log: bool,
    pub block_tasks_log: bool,
    pub unblock_tasks_log: bool,
    pub task_exit_log: bool,
    pub resource_lock_log: bool,
    pub resource_unlock_log: bool,
    pub message_broadcast_log: bool,
    pub message_recieve_log: bool,
    pub semaphore_signal_log: bool,
    pub semaphore_reset_log: bool,
    pub timer_event_log: bool,
}
// use a circular queue instead of this crap.
// ensure the handler is not None in start_kernel.
impl Logger {
    pub const fn new() -> Self {
        Self {
            logs: [None; MAX_LOGS],
            start: 0,
            end: 0,
            release_log : false,
            block_tasks_log : false,
            unblock_tasks_log : false,
            task_exit_log : false,
            resource_lock_log : false,
            resource_unlock_log : false,
            message_broadcast_log : false,
            message_recieve_log : false,
            semaphore_signal_log : false,
            semaphore_reset_log : false,
            timer_event_log : false,
        }
    }
    pub fn push(&mut self, event: LogEvent) {
        self.logs[self.end] = Some(event);
        self.end = (self.end+1)%MAX_LOGS;
        if self.start == self.end {
            self.start = (self.start+1)%MAX_LOGS;
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
        let pos = self.start;
        let val = self.logs[pos];
        self.logs[pos] = None;
        self.start = (self.start+1)%MAX_LOGS;
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
