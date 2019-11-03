#[derive(Clone, Copy)]
pub struct Time {
    pub m_sec_10: u32,
    pub sec: u32,
    pub min: u32,
    pub hour: u32,
    pub day: u32,
}

pub enum TickType {
    MilliSec10,
    Sec,
    Min,
    Hour,
    Day,
}

impl Time {
    pub const fn new() -> Self {
        Self {
            m_sec_10: 0,
            sec: 0,
            min: 0,
            hour: 0,
            day: 0,
        }
    }

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
