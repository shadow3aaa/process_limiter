use crate::{LimitInfo, Task, TaskStatus};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use sysinfo::{System};

pub trait TaskExt {
    fn new(system: Arc<Mutex<System>>) -> Self;
    fn pause(&mut self) -> Result<(), Box<dyn Error>>;
    fn start(&mut self) -> Result<(), Box<dyn Error>>;
    fn restrart(&mut self) -> Result<(), Box<dyn Error>>;
    fn status(&self) -> TaskStatus;
    fn with_re_search(self) -> Self;
}

impl TaskExt for Task {
    fn new(system: Arc<Mutex<System>>) -> Self {
        let thread = thread::spawn(move || {});
        // By default, threads are paused
        Self {
            system,
            thread,
            re_search: false,
            status: TaskStatus::Init,
            info: LimitInfo::default(),
        }
    }
    fn pause(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn restrart(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn status(&self) -> TaskStatus {
        todo!()
    }
    fn with_re_search(mut self) -> Self {
        self.re_search = true;
        self
    }
}

fn pause_thread(_thread: &mut JoinHandle<()>) {
    todo!()
}
