use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut _hour = hours % 24;
        if _hour < 0 { _hour += 24 }
        let new = Self { hours: _hour, minutes: 0 };
        new.add_minutes(minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let sum_minutes = self.minutes + minutes;
        let mut _hour = (sum_minutes / 60 + self.hours) % 24;
        let mut _minutes = sum_minutes % 60;
        if _minutes < 0 { 
            _minutes += 60; 
            _hour -= 1;
        }
        if _hour < 0 { _hour += 24 }
        Self { hours: _hour, minutes: _minutes }
    }
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
