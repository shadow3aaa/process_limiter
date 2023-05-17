mod core;
mod display;
mod ext;

pub use crate::core::*;
pub use display::*;
pub use ext::{limiter::*, task::*};
use std::{
    collections::HashMap,
    marker::PhantomData,
    sync::{mpsc::Sender, Arc, Mutex},
    thread::JoinHandle,
    time::Duration,
};
use sysinfo::{Pid, Process, Signal, System, SystemExt};

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

// The representative requests an update from the system
// Contains a U32 representing a Pid
pub(crate) enum Update {
    All,
    Spec(Pid),
}

pub enum TaskStatus {
	Init,      // 初始化中
    Active,    // 限制进行中
    Paused,    // 限制已暂停并且暂时取消限制
    Expired    // 跟踪的pid已经消失/被复用
}

#[derive(Debug)]
pub struct Limiter<'a: 'b, 'b> {
    system: Arc<Mutex<System>>,
    // This allows the system to handle update requests sent by Tasks
    updater_thread: JoinHandle<()>,
    sender_orginal: Sender<Update>,

    // Save the map of Tasks in Limiter
    tasks: HashMap<Pid, Task<'b>>,

    // _marker is used to transmit <'a> life cycle to Limiter
    // Makes sense only for the compiler
    _marker: PhantomData<&'a ()>,
}

#[derive(Debug)]
pub struct Task<'a> {
    process: &'a Process,
    sender: Sender<Update>,
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
