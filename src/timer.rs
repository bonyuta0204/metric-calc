use std::{fmt, time::Instant};

pub struct Timer {
    instant: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            instant: Instant::now(),
        }
    }
    pub fn elapsed_ms_str(&self) -> String {
        format!("{:4}", self.instant.elapsed().as_millis())
    }
}
