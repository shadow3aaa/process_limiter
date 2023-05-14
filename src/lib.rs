mod core;
mod display;
mod ext;

pub use crate::core::*;
pub use display::*;
pub use ext::{limiter::*, task::*};
use std::{collections::HashMap, marker::PhantomData, time::Duration};
use sysinfo::{Pid, Process, System, SystemExt, Signal};

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

#[derive(Debug)]
pub struct Limiter<'a: 'b, 'b> {
    system: System,
    tasks: HashMap<Pid, Task<'b>>,

    // _marker is used to transmit <'a> life cycle to Limiter
    // Makes sense only for the compiler
    _marker: PhantomData<&'a ()>,
}

#[derive(Debug)]
pub struct Task<'a> {
    process: &'a Process,
    info: LimitInfo
}

#[derive(Debug)]
pub struct LimitInfo {
    current_usage: f32,
    target_usage: f32,
    last_work_slice: Duration,
    total_slice: Duration,
}