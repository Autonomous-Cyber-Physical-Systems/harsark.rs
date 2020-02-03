use core::cell::RefCell;

use crate::KernelError;
use crate::priv_execute;
use crate::system::scheduler::*;
use crate::utils::arch::{svc_call,Mutex,critical_section};
use crate::utils::helpers::is_privileged;
use crate::system::logger::*;

static Logger: Mutex<RefCell<Logger>> = Mutex::new(RefCell::new(Logger::new()));

pub fn report(event_type: LogEventType) {
    critical_section(|cs_token| {
        // use actual timer.
        Logger.borrow(cs_token).borrow_mut().push(LogEvent::new(event_type, 0));
    })
}

pub fn process<F> (handler: F) 
where
    F: Fn(LogEvent),
{
    critical_section(|cs_token| {
        while let Some(event) = Logger.borrow(cs_token).borrow_mut().pop() {
            handler(event);
        }
    })
}