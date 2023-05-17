use crate::{Task, TaskStatus};
use std::time::Duration;
use std::sync::mpsc::Sender;
use sysinfo::Process;

pub trait TaskExt<'a> {
    fn new(process: &'a Process, sender: Sender) -> Self;
    fn stop_limit(&mut self) -> Result<(), &'static str>;
    fn start_limit(&mut self) -> Result<(), &'static str>;
    fn status(&self) -> TaskStatus;
}

impl<'a> TaskExt<'a> for Task<'a> {
    fn new(process: &'a Process, sender: Sender) -> Self {
        Self {
            process,
            sender,
            status: TaskStatus::Init,
        }
    }
}
