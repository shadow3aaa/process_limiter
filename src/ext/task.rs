use crate::{LimitInfo, Task, TaskStatus, process};
use std::error::Error;
use std::os::unix::thread::JoinHandleExt;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use sysinfo::{System, SystemExt, Pid, PidExt, ProcessExt, Signal};

pub trait TaskExt {
    fn new(system: Arc<Mutex<System>>) -> Self;
    fn start(&mut self, pid: u32) -> Result<(), Box<dyn Error>>;
    fn pause(&mut self) -> Result<(), Box<dyn Error>>;
    fn remuse(&mut self) -> Result<(), Box<dyn Error>>;
    fn status(&self) -> TaskStatus;
}

impl TaskExt for Task {
    fn new(system: Arc<Mutex<System>>) -> Self {
        Self {
            system,
            pid: None,
            thread: None,
            status: TaskStatus::Init,
            info: LimitInfo::default(),
        }
    }
    fn start(&mut self, pid: u32) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn pause(&mut self) -> Result<(), Box<dyn Error>> {
        let pid = if let Some(o) = &self.thread {
            o.as_pthread_t() as u32
        } else {
            return Err("Limiter process does not exist".into());
        };
        let mut system = if let Ok(o) = self.system.lock() {
            o
        } else {
            return Err("Failed to get system".into());
        };
        stop_by_pid(&mut system, pid)
    }
    fn remuse(&mut self) -> Result<(), Box<dyn Error>> {
        let pid = if let Some(o) = &self.thread {
            o.as_pthread_t() as u32
        } else {
            return Err("Limiter process does not exist".into());
        };
        let mut system = if let Ok(o) = self.system.lock() {
            o
        } else {
            return Err("Failed to get system".into());
        };
        cont_by_pid(&mut system, pid)
    }
    fn status(&self) -> TaskStatus {
        todo!()
    }
}

fn stop_by_pid(system: &mut System, pid: u32) -> Result<(), Box<dyn Error>> {
    let process = if let Some(o) = system.process(Pid::from_u32(pid)) {
        o
    } else {
        return Err("Process not found".into());
    };
    process.kill_with(Signal::Stop).expect("not supported");
    Ok(())
}

fn cont_by_pid(system: &mut System, pid: u32) -> Result<(), Box<dyn Error>> {
    let process = if let Some(o) = system.process(Pid::from_u32(pid)) {
        o
    } else {
        return Err("Process not found".into());
    };
    process.kill_with(Signal::Continue).expect("not supported");
    Ok(())
}