use crate::{core, Task};
use sysinfo::Process;

pub trait TaskExt<'b> {
    fn new(process: &'b Process) -> Self;
}

impl<'b> TaskExt<'b> for Task<'b> {
    fn new(process: &'b Process) -> Self {
        Self { process }
    }
}
