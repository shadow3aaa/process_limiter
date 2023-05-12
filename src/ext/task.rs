use crate::{Task, LimitInfo, core::limit_info::*};
use sysinfo::Process;
use std::time::Duration;

pub trait TaskExt<'b> {
    fn new(process: &'b Process) -> Self;
    fn limit_once(&self) ->
}

impl<'b> TaskExt<'b> for Task<'b> {
    fn new(process: &'b Process) -> Self {
        let info = LimitInfo::new(: f32, target_usage: f32, last_work_slice: Duration)
        Self {
            process,
            info : 
        }
    }
}
