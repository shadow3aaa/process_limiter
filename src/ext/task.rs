use crate::{LimitInfo, Task, TaskStatus, Update};
use std::error::Error;
use std::sync::{Mutex, Arc};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use sysinfo::{Process, System};

pub trait TaskExt<'a> {
    fn new(process: &'a Process, system: Arc<Mutex<System>>) -> Self;
    fn stop_limit(&mut self) -> Result<(), Box<dyn Error>>;
    fn start_limit(&mut self) -> Result<(), Box<dyn Error>>;
    fn status(&self) -> TaskStatus;
}

impl<'a> TaskExt<'a> for Task<'a> {
    fn new(process: &'a Process, system: Arc<Mutex<System>>) -> Self {
        let thread = thread::spawn(|| {
            
        });
        // By default, threads are paused
        Self {
            process,
            thread,
            system,
            status: TaskStatus::Init,
            info: LimitInfo::default(),
        }
    }
    fn stop_limit(&mut self) -> Result<(), Box<dyn Error>> {
        todo!();
        Ok(())
    }

    fn start_limit(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn status(&self) -> TaskStatus {
        todo!()
    }
}

fn pause_thread(thread: &mut JoinHandle<()>) {
    todo!()
}
