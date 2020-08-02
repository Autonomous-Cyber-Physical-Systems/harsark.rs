use crate::config::MAX_TASKS;
use crate::system::scheduler::TaskId;
use crate::kernel::logging;
use crate::system::system_logger::LogEventType;

pub struct TaskMonitor {
    active_deadlines: [Option<u32>; MAX_TASKS],
    handler: Option<fn()>,
}

impl TaskMonitor {
    pub const fn new() -> Self{
        Self {
            active_deadlines: [None; MAX_TASKS],
            handler: None,
        }
    }
    pub fn set_deadline(&mut self, tid: TaskId, abs_deadline: u32) {
        self.active_deadlines[tid as usize] = Some(abs_deadline);
    }
    pub fn set_handler(&mut self, handler: fn()) {
        self.handler = Some(handler);
    }
    pub fn clear_deadline(&mut self, tid: TaskId) {
        self.active_deadlines[tid as usize] = None;
    }
    pub fn sweep_deadlines(&mut self, curr_time: u32) {
        for tid in 0..MAX_TASKS {
            if let Some(deadline) = self.active_deadlines[tid] {
                if deadline == curr_time {
                    self.active_deadlines[tid] = None;
                    if self.handler.is_some() {
                        (self.handler.unwrap())();
                    }
                }
            }
        }
    }
}