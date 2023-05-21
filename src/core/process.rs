use crate::LimitInfo;
use std::time::Duration;
use sysinfo::{Process, ProcessExt, Signal};
use spin_sleep::sleep;

// Use a run-time-slice limiting approach
// Calculation method: (Working time slice percentage) = (Process CPU usage) / (Target CPU usage) * (Last working time slice percentage)
// ⚠: If the calculation Result% > 100% time slice, it is considered 100%
// ⚠: If the calculation Result% < Minimum margin% time slice, then it is considered 0%

pub fn limit_process(process: &Process, info: &mut LimitInfo) -> Duration {
    let (work_t, sleep_t, _) = info.result();
    sleep(work_t);
    process.kill_with(Signal::Stop).unwrap();
    sleep(sleep_t);
    process.kill_with(Signal::Continue).unwrap();
    work_t
}
