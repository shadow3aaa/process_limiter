use crate::{LimitInfo, Task, TaskStatus, core, misc};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

use sysinfo::{System, SystemExt, Pid, PidExt, ProcessExt, ProcessRefreshKind};

// What the Process needs to be refreshed
macro_rules! PROCESS_REFRESH {
    () => {
        ProcessRefreshKind::new().with_cpu().with_user()
    };
}

pub trait TaskExt {
    fn new(system: Arc<Mutex<System>>) -> Self;
    fn start(&mut self, pid: u32);
    fn pause(&mut self) -> Result<(), Box<dyn Error>>;
    fn remuse(&mut self) -> Result<(), Box<dyn Error>>;
    fn set_target(&mut self, new_target: f32) -> Result<(), Box<dyn Error>>;
    fn status(&mut self) -> TaskStatus;
}

impl TaskExt for Task {
    fn new(system: Arc<Mutex<System>>) -> Self {
        Self {
            system,
            thread: None,
            target: Arc::new(Mutex::new(1.0)),
        }
    }
    fn start(&mut self, pid: u32) {
        let target = self.target.clone();
        let system = self.system.clone();
        self.thread = Some(thread::spawn(move || {
            let mut info = LimitInfo::default();
            loop {
                let mut system = if let Ok(o) = system.lock() {
                    o
                } else {
                    continue;
                };
                if !system.refresh_process_specifics(Pid::from_u32(pid), PROCESS_REFRESH!()) {
                    break;
                };
                let process = if let Some(o) = system.process(Pid::from_u32(pid)) {
                    o
                } else {
                    break;
                };

                info.update_current_usage(process.cpu_usage());
                if let Ok(o) = target.lock() {
                    info.update_taregt_usage(*o);
                } else {
                    continue;
                }
                let work_slice = core::process::limit_process(process, &mut info);
                info.update_work_slice(work_slice);
            }
        }));
    }
    fn pause(&mut self) -> Result<(), Box<dyn Error>> {
        let pid = self.get_thread_pid()?;
        let mut system = if let Ok(o) = self.system.lock() {
            o
        } else {
            return Err("Failed to get system".into());
        };
        misc::stop_by_pid(&mut system, pid)
    }
    fn remuse(&mut self) -> Result<(), Box<dyn Error>> {
        let pid = self.get_thread_pid()?;
        let mut system = if let Ok(o) = self.system.lock() {
            o
        } else {
            return Err("Failed to get system".into());
        };
        misc::cont_by_pid(&mut system, pid)
    }
    fn set_target(&mut self, new_target: f32) -> Result<(), Box<dyn Error>> {
        let mut target = match self.target.lock() {
            Ok(o) => o,
            Err(_e) => {
                return Err("Failed to get the lock".into());
            }
        };
        *target = new_target;
        Ok(())
    }
    fn status(&mut self) -> TaskStatus {
        let pid = if let Ok(o) = self.get_thread_pid() {
            o
        } else {
            return TaskStatus::NeedInit;
        };
        let mut system = if let Ok(o) = self.system.lock() {
            o
        } else {
            return TaskStatus::NeedInit;
        };
        match misc::status_by_pid(&mut system, pid) {
            Ok(o) => o,
            Err(_) => TaskStatus::NeedInit
        }
    }
}

impl Task {
    fn get_thread_pid(&mut self) -> Result<u32, Box<dyn Error>> {
        if self.thread.is_none() {
            return Err("Limiter process does not exist".into());
        }
        if let Some(o) = &self.thread {
            Ok(misc::get_thread_pid(o))
        } else {
            Err("Limiter process does not exist".into())
        }
    }
}