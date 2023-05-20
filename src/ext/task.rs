use crate::{LimitInfo, Task, TaskStatus};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use sysinfo::{System};

pub trait TaskExt {
    fn new(system: Arc<Mutex<System>>) -> Self;
    fn start(&mut self, pid: u32) -> Result<(), Box<dyn Error>>;
    fn pause(&mut self) -> Result<(), Box<dyn Error>>;
    fn restrart(&mut self, new_pid: u32) -> Result<(), Box<dyn Error>>;
    fn status(&self) -> TaskStatus;
}

impl TaskExt for Task {
    fn new(system: Arc<Mutex<System>>) -> Self {
        let thread = thread::spawn(move || {});
        // By default, threads are paused
        Self {
            system,
            pid: None,
            thread,
            status: TaskStatus::Init,
            info: LimitInfo::default(),
        }
    }
    fn start(&mut self, pid: u32) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn pause(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn restrart(&mut self, new_pid: u32) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn status(&self) -> TaskStatus {
        todo!()
    }
}

fn pause_thread(_thread: &mut JoinHandle<()>) {
    todo!()
}
