use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut hours = self.minutes / 60;
        let mut mins = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, mins)
    }
}

impl Clock {
    pub fn new(hour: i32, minute: i32) -> Self {
        Clock::build(hour * 60 + minute)
    }

    fn build(minutes: i32) -> Self {
        let mut mins = minutes;
        while mins < 0 {
            mins += 1440;
        }
        let clock = Clock {
            minutes: mins % 1440,
        };
        clock
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        Clock::build(self.minutes + minutes)
    }
}