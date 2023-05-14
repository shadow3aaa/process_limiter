mod core;
mod display;
mod ext;

pub use core::*;
pub use display::*;
pub use ext::{limiter::*, task::*};
use std::{collections::HashMap, marker::PhantomData, time::Duration};
use sysinfo::{Pid, Process, System};

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