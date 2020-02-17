use crate::config::MAX_TASKS;
use crate::system::scheduler::TaskId;
use crate::kernel::logging;
use crate::system::system_logger::LogEventType;
use cortex_m_semihosting::hprintln;

pub struct ProcessMonitor {
    active_deadlines: [Option<u32>; MAX_TASKS]
}

impl ProcessMonitor {
    pub const fn new() -> Self{
        Self {
            active_deadlines: [None; MAX_TASKS]
        }
    }
    pub fn set_deadline(&mut self, tid: TaskId, abs_deadline: u32) {
        self.active_deadlines[tid as usize] = Some(abs_deadline);
    }
    pub fn clear_deadline(&mut self, tid: TaskId) {
        self.active_deadlines[tid as usize] = None;
    }
    pub fn sweep_deadlines(&mut self, curr_time: u32) {
        for tid in 0..MAX_TASKS {
            if let Some(deadline) = self.active_deadlines[tid] {
                if deadline == curr_time {
                    if logging::get_release() {
                        logging::report(LogEventType::DeadlineExpired(tid as TaskId, 0));
                    }
                    self.active_deadlines[tid] = None;
                }
            }
        }
    }
}