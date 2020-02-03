use crate::config::MAX_TASKS;
use crate::system::scheduler::TaskId;
use crate::kernel::logging;
use crate::system::logger::LogEventType;
use cortex_m_semihosting::hprintln;

pub struct ProcessMonitor {
    timer: u32,
    active_deadlines: [Option<u32>; MAX_TASKS]
}

impl ProcessMonitor {
    pub const fn new() -> Self{
        Self {
            timer: 0,
            active_deadlines: [None; MAX_TASKS]
        }
    }
    pub fn set_deadline(&mut self, tid: TaskId, deadline: u32) {
        self.active_deadlines[tid as usize] = Some(self.timer + deadline);
    }
    pub fn clear_deadline(&mut self, tid: TaskId) {
        self.active_deadlines[tid as usize] = None;
    }
    pub fn update_timer(&mut self) {
        self.timer += 1;
        for tid in 0..MAX_TASKS {
            if let Some(deadline) = self.active_deadlines[tid] {
                if deadline == self.timer {
                    // hprintln!("{}", tid);
                    if logging::get_release_log() {
                        logging::report(LogEventType::DeadlineExpired(tid as TaskId, 0));
                    }
                    self.active_deadlines[tid] = None;
                }
            }
        }
    }
}