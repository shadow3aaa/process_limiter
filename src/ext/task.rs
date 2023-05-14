use crate::{Task};
use sysinfo::Process;
use std::time::Duration;

pub trait TaskExt<'b> {
    fn new(process: &'b Process) -> Self;
    fn limit_once(&self) ->
}

impl<'b> TaskExt<'b> for Task<'b> {
    fn new(process: &'b Process) -> Self {
        
    }
}
