//! # Time Manager
//!
//! Defines Data-structures which manage Kernel time.

/// This struct represents a time object, i.e., it stores the time format.
#[derive(Clone, Copy)]
pub struct Time {
    pub m_sec_10: u32,
    pub sec: u32,
    pub min: u32,
    pub hour: u32,
    pub day: u32,
}

/// This enum represents the highest order time that elapsed in a tick.
pub enum TickType {
    MilliSec10,
    Sec,
    Min,
    Hour,
    Day,
}

impl Time {
    /// Returns a new instance of `Time`.
    pub const fn new() -> Self {
        Self {
            m_sec_10: 0,
            sec: 0,
            min: 0,
            hour: 0,
            day: 0,
        }
    }

    /// A tick updates the Time object’s `m_sec_10` field, which implies 10 MilliSecond has passed,
    /// and After every tick, the other fields (Second, Minutes, Hour, and Day) of the Time object
    /// are also updated if required. The tick method is called by the SysTick interrupt handler, and
    /// the return value is used by the interrupt handler to dispatch events. Note that if returned TickType
    /// is Hour, it not only implies the current tick caused completion of an hour, but that is caused
    /// completion of an Hour, Second, and 10 MilliSecond but not a Day.
    pub fn tick(&mut self) -> TickType {
        self.m_sec_10 += 1;
        let mut res = TickType::MilliSec10;

        if self.m_sec_10 == 100 {
            self.m_sec_10 = 0;
            self.sec += 1;
            res = TickType::Sec;
        }

        if self.sec == 60 {
            self.sec = 0;
            self.min += 1;
            res = TickType::Min;
        }

        if self.min == 60 {
            self.min = 0;
            self.hour += 1;
            res = TickType::Hour;
        }

        if self.hour == 24 {
            self.hour = 0;
            self.day += 1;
            res = TickType::Day;
        }

        // Should be updated if there is any better implementation
        if self.day == core::u32::MAX {
            self.day = 0;
        }

        res
    }
}
