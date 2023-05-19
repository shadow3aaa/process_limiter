use crate::{LimitInfo, Task, TaskStatus, Update};
use std::error::Error;
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle, Thread};
use std::time::Duration;
use sysinfo::Process;

pub trait TaskExt<'a> {
    fn new(process: &'a Process, sender: Sender<Update>) -> Self;
    fn stop_limit(&mut self) -> Result<(), Box<dyn Error>>;
    fn start_limit(&mut self) -> Result<(), Box<dyn Error>>;
    fn status(&self) -> TaskStatus;
}

impl<'a> TaskExt<'a> for Task<'a> {
    fn new(process: &'a Process, sender: Sender<Update>) -> Self {
        let thread = thread::spawn(|| {
            
        });
        // By default, threads are paused
        thread.thread().park();
        Self {
            process,
            thread,
            sender,
            status: TaskStatus::Init,
            info: LimitInfo::default(),
        }
    }
    fn stop_limit(&mut self) -> Result<(), Box<dyn Error>> {
        self.thread.thread().park()?;
        Ok(())
    }
}

fn pause_thread(thread: &mut JoinHandle<()>) {
    thread.thread().park()
}
