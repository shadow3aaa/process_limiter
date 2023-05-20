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
    // Result:
    // 0: Work time, 1: Sleep time, 2: Total time
    // Calculation method: (Working time slice percentage) = (Process CPU usage) / (Target CPU usage) * (Last working time slice percentage)
    pub fn result(&mut self) -> (Duration, Duration, Duration) {
        let mut work_slice_per = self.current_usage / self.target_usage;
        // ⚠: If the calculation Result% > 100% time slice, it is considered 100%
        // ⚠: If the calculation Result% < 0% time slice, then it is considered 0%
        if work_slice_per > 1.0 {
            work_slice_per = 1.0;
        } else if work_slice_per < 0.0 {
            work_slice_per = 0.0;
        }

        let work_slice = self.last_work_slice.mul_f32(work_slice_per);
        let sleep_slice = self.total_slice - work_slice;
        (work_slice, sleep_slice, self.total_slice)
    }
}
