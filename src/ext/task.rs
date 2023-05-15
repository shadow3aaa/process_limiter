use crate::Task;
use std::time::Duration;
use sysinfo::Process;

pub trait TaskExt<'b> {
    fn new(process: &'b Process) -> Self;
}

impl<'b> TaskExt<'b> for Task<'b> {
    fn new(process: &'b Process) -> Self {}
}
