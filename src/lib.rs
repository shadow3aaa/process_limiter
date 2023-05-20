mod core;
mod ext;

pub use crate::core::*;
pub use ext::{limiter::*, task::*};
use std::{
    sync::{Arc, Mutex},
    thread::JoinHandle,
    time::Duration,
};
use sysinfo::{Signal, System, SystemExt};

// 检测是否支持该库的方法
pub fn support() -> bool {
    if !System::IS_SUPPORTED {
        return false;
    }
    if !System::SUPPORTED_SIGNALS.contains(&Signal::Stop)
        || !System::SUPPORTED_SIGNALS.contains(&Signal::Continue)
    {
        return false;
    }
    true
}

#[derive(Debug)]
pub enum TaskStatus {
    Init,    // 初始化中
    Active,  // 限制进行中
    Paused,  // 限制已暂停并且暂时取消限制
    Expired, // 跟踪的pid已经消失/被复用
}

#[derive(Debug)]
pub struct Limiter {
    system: Arc<Mutex<System>>,
}

#[derive(Debug)]
pub struct Task {
    system: Arc<Mutex<System>>,
    thread: JoinHandle<()>,
    re_search: bool,
    status: TaskStatus,
    info: LimitInfo,
}

#[derive(Debug)]
pub struct LimitInfo {
    current_usage: f32,
    target_usage: f32,
    last_work_slice: Duration,
    total_slice: Duration,
}
