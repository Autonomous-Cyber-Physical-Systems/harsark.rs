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
    ResourceLock(TaskId),
    ResourceUnlock(TaskId),
    MessageBroadcast(BooleanVector),
    MessageRecieve(TaskId),
    SemaphoreSignal(BooleanVector,BooleanVector),
    SemaphoreReset(TaskId),
    TimerEvent(EventId),
    DeadlineExpired(TaskId,u32),
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

pub struct SystemLogger {
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
impl SystemLogger {
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
            LogEventType::BlockTasks(tasks_mask) => write!(f, "BlockTasks"),
            LogEventType::UnblockTasks(tasks_mask) => write!(f, "UnblockTasks"),
            LogEventType::TaskExit(tasks_mask) => write!(f, "TaskExit"),
            LogEventType::ResourceLock(ceiling) => write!(f, "ResourceLock"),
            LogEventType::ResourceUnlock(ceiling) => write!(f, "ResourceUnlock"),
            LogEventType::MessageBroadcast(recievers) => write!(f, "MessageBroadcast"),
            LogEventType::MessageRecieve(task_id) => write!(f, "MessageRecieve"),
            LogEventType::SemaphoreSignal(tasks_released,tasks_notified) => write!(f, "SemaphoreSignal"),
            LogEventType::SemaphoreReset(task_id) => write!(f, "SemaphoreReset"),
            LogEventType::TimerEvent(EventId) => write!(f, "TimerEvent"),
            LogEventType::DeadlineExpired(TaskId, u32) => write!(f, "DeadlineExpired"),
        }
    }
}
