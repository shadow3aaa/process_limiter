//! # Examples
//!
//! ```
//! // Create a Limiter that generates Tasks that are actually used for control.
//! use task_limiter::{Limiter, LimiterExt};
//! let mut limiter = Limiter::new();
//! // Derive a Task from Limiter
//! let mut task = limiter.spawn();
//! /* Check the documentation for the Task structure to learn how to use this Task */
//! ```

mod core;
mod ext;
pub(crate) mod misc;
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
    NeedInit,
    Active,
    Paused,
}

#[derive(Debug)]
pub struct Limiter {
    system: Arc<Mutex<System>>,
}

#[derive(Debug)]
pub struct Task {
    system: Arc<Mutex<System>>,
    thread: Option<JoinHandle<()>>,
    target: Arc<Mutex<f32>>,
}

#[derive(Debug)]
pub struct LimitInfo {
    current_usage: f32,
    target_usage: f32,
    last_work_slice: Duration,
    total_slice: Duration,
}
