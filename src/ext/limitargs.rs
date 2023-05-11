use std::time::Duration;
use crate::{Limiter, Task, LimitArgs};
use sysinfo::Process;

const TIME_SLICE = Duration::from_millis(150);

pub trait LimitArgsExt<'a> {
    fn new(process: &'a Process，target_usage: f32) -> Self;
}

impl<'a> LimitArgsExt for LimitArgs<'a> {
    fn fn new(process: &'a Process，target_usage: f32) -> Self {
        LimitArgs<'a> {
            process: &'a Process,
            current_usage: f32,
            target_usage: f32,
            time_slice: TIME_SLICE,
        }
    }
}