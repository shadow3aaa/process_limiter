use std::{thread::sleep, time::Duration};
use sysinfo::{Process, ProcessExt, System, SystemExt, Signal};
use crate::LimitInfo;

// Use a run-time-slice limiting approach
// Calculation method: (Working time slice percentage) = (Process CPU usage) / (Target CPU usage) * (Last working time slice percentage)
// ⚠: If the calculation Result% > 100% time slice, it is considered 100%
// ⚠: If the calculation Result% < Minimum margin% time slice, then it is considered 0%

pub fn limit_process(process: &Process, mut info: LimitInfo) -> Duration {
    let (work_slice, sleep_slice, total_slice) = info.result();
    sleep(work_slice);
    process.kill_with(Signal::Stop);
    sleep(sleep_slice);
    process.kill_with(Signal::Continue);
    work_slice
}

// 检测是否支持该库的方法
pub fn support() -> bool {
    if !System::IS_SUPPORTED {
        return false
    }
    if !System::SUPPORTED_SIGNALS.contains(&Signal::Stop) || !System::SUPPORTED_SIGNALS.contains(&Signal::Continue) {
        return false
    }
    true
}