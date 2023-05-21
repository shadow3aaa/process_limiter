use crate::LimitInfo;
use std::time::Duration;
const TOTAL_SLICE: Duration = Duration::from_millis(100);

// In order to calculate the working time slice and sleep time slice length, these data is needed:
// Current CPU usage of the process, target CPU usage, last working time

impl Default for LimitInfo {
    fn default() -> Self {
        Self {
            current_usage: 0.0,
            target_usage: 0.0,
            last_work_slice: TOTAL_SLICE,
            total_slice: TOTAL_SLICE,
        }
    }
}

impl LimitInfo {
    pub fn new(current_usage: f32, target_usage: f32, last_work_slice: Duration) -> Self {
        LimitInfo {
            current_usage,
            target_usage,
            last_work_slice,
            total_slice: TOTAL_SLICE,
        }
    }
    // The default is 100ms, if you need to customize it
    // However I don't think it's necessary, maybe later
    /* pub fn spec_total_slice(mut self, total_slice: Duration) -> Self {
        self.total_slice = total_slice;
        self
    } */
    pub fn update_current_usage(&mut self, usage: f32) {
        self.current_usage = usage;
    }
    pub fn update_taregt_usage(&mut self, usage: f32) {
        self.target_usage = usage;
    }
    pub fn update_work_slice(&mut self, work_slice: Duration) {
        self.last_work_slice = work_slice;
    }
    pub fn total_slice(&self) -> Duration {
        self.total_slice
    }
    pub fn result(&mut self) -> (Duration, Duration, Duration) {
        let max = self.total_slice.as_nanos() as f32 / self.last_work_slice.as_nanos() as f32;
        let mut work = self.target_usage / self.current_usage;
        if work.gt(&max) {
            work = max;
        }
        let mut work = self.last_work_slice.mul_f32(work);
        if work > self.total_slice {
            work = self.total_slice;
        }
        let sleep = self.total_slice - work;
        (work, sleep, self.total_slice)
    }
}

use std::fmt::Display;
use std::fmt::Formatter;
impl Display for LimitInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "usage: {}", self.current_usage)?;
        writeln!(f, "target: {}", self.target_usage)?;
        writeln!(f, "last worked time: {:?}", self.last_work_slice)?;
        Ok(())
    }
}