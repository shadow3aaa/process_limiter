use crate::Task;
use sysinfo::{
    Process,
};

impl<'sys> Task<'sys> {
    pub fn new(process: &'sys Process) -> Self {
        Self {
            process,
        }
    }
}