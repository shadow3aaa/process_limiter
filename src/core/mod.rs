use crate::LimitArgs;
use std::time::Duration;
use sysinfo::{Pid, PidExt, Process, ProcessExt, System, SystemExt};

// Use a run-time-slice limiting approach
// Calculation method: (Working time slice percentage) = (Process CPU usage) / (Target CPU usage) * (Last working time slice percentage)
// ⚠: If the calculation Result% > 100% time slice, it is considered 100%
// ⚠: If the calculation Result% < Minimum margin% time slice, then it is considered 0%

pub fn limit_process(spec_limit: LimitArgs) {}
