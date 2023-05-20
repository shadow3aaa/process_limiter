use crate::TaskStatus;
use std::{error::Error, os::unix::thread::JoinHandleExt, thread::JoinHandle};
use sysinfo::{Pid, PidExt, ProcessExt, ProcessStatus, Signal, System, SystemExt};

pub fn get_thread_pid<T>(thread: &JoinHandle<T>) -> u32 {
    thread.as_pthread_t() as u32
}

pub fn stop_by_pid(system: &mut System, pid: u32) -> Result<(), Box<dyn Error>> {
    let process = if let Some(o) = system.process(Pid::from_u32(pid)) {
        o
    } else {
        return Err("Process not found".into());
    };
    process.kill_with(Signal::Stop).expect("not supported");
    Ok(())
}

pub fn cont_by_pid(system: &mut System, pid: u32) -> Result<(), Box<dyn Error>> {
    let process = if let Some(o) = system.process(Pid::from_u32(pid)) {
        o
    } else {
        return Err("Process not found".into());
    };
    process.kill_with(Signal::Continue).expect("not supported");
    Ok(())
}

pub fn status_by_pid(system: &mut System, pid: u32) -> Result<TaskStatus, Box<dyn Error>> {
    let process = if let Some(o) = system.process(Pid::from_u32(pid)) {
        o
    } else {
        return Err("Process not found".into());
    };
    Ok(match process.status() {
        ProcessStatus::Run => TaskStatus::Active,
        ProcessStatus::Sleep => TaskStatus::Active,
        ProcessStatus::Stop => TaskStatus::Paused,
        _ => TaskStatus::NeedInit,
    })
}
