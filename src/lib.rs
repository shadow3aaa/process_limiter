mod core;
mod display;
mod ext;

pub use display::*;
pub use ext::{limiter::*, task::*};
use std::{collections::HashMap, marker::PhantomData, time::Duration};
use sysinfo::{Process, System};

#[derive(Debug)]
pub struct Limiter<'a: 'b, 'b> {
    system: System,
    tasks: HashMap<u32, Task<'b>>,

    // _marker is used to transmit <'a> life cycle to Limiter
    // Makes sense only for the compiler
    _marker: PhantomData<&'a ()>,
}

#[derive(Debug)]
pub struct Task<'a> {
    process: &'a Process,
}

#[derive(Debug)]
pub struct LimitArgs<'a> {
    process: &'a Process,
    current_usage: f32,
    target_usage: f32,
    time_slice: Duration,
}
