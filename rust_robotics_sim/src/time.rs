use chrono::{Local, NaiveTime};

pub struct Timer {
    start: NaiveTime,
    previous: NaiveTime,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            start: Local::now().time(),
            previous: Local::now().time(),
        }
    }
}

impl Timer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn tick(&mut self) -> f64 {
        let dt = self.dt();
        self.previous = Local::now().time();
        dt
    }
    pub fn fps(&self) -> f64 {
        1.0 / self.dt()
    }
    pub fn dt(&self) -> f64 {
        (Local::now().time() - self.previous)
            .num_microseconds()
            .unwrap_or(0) as f64
            / 1.0e6
    }
    pub fn elapsed(&self) -> f64 {
        (self.start - Local::now().time())
            .num_microseconds()
            .unwrap_or(0) as f64
            / 1.0e6
    }
}
