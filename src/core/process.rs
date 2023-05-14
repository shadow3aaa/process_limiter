use sysinfo::{Process, ProcessExt, System, SystemExt, Signal};

// Use a run-time-slice limiting approach
// Calculation method: (Working time slice percentage) = (Process CPU usage) / (Target CPU usage) * (Last working time slice percentage)
// ⚠: If the calculation Result% > 100% time slice, it is considered 100%
// ⚠: If the calculation Result% < Minimum margin% time slice, then it is considered 0%

pub fn limit_process(process: &Process, info: LimitInfo) -> Result<Duration, ()> {
    let (work_slice, sleep_slice, total_slice) = info.result();
    process.
}

// 检测是否支持该库的方法
pub fn support() -> bool {
    if !System::IS_SUPPORTED {
        false
    }
    if !System::SUPPORTED_SIGNALS.contains(Signal::Stop) || !System::SUPPORTED_SIGNALS.contains(Signal::Continue) {
        false
    }
    true
}