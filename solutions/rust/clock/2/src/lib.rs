use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Clock {
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hh = (self.minutes / 60).rem_euclid(24).abs();
        let mm = self.minutes.rem_euclid(60).abs();

        write!(f, "{:02}:{:02}", hh, mm)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        (Self::default()).add_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: (self.minutes + minutes).rem_euclid(1440),
        }
    }
}
