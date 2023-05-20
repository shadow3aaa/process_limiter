use crate::{Limiter, Task, TaskExt};
use std::{
    sync::{Arc, Mutex},
};
use sysinfo::{
    Pid, PidExt, ProcessRefreshKind, RefreshKind, System, SystemExt, ProcessExt,
};

// What the Process needs to be refreshed
macro_rules! PROCESS_REFRESH {
    () => {
        ProcessRefreshKind::new().with_cpu().with_user()
    };
}

impl Default for Limiter {
    fn default() -> Self {
        let system = Arc::new(Mutex::new(System::new_with_specifics(
            RefreshKind::new().with_processes(PROCESS_REFRESH!()),
        )));

        Self { system }
    }
}

pub trait LimiterExt {
    fn new() -> Self;
    fn spawn(&mut self) -> Task;
    fn search_pid(&mut self, name: &str) -> Option<Vec<u32>>;
    fn search_pid_exactly(&mut self, name: &str) -> Option<u32>;
}

impl LimiterExt for Limiter {
    fn new() -> Self {
        Self::default()
    }
    fn spawn(&mut self) -> Task {
        Task::new(self.system.clone())
    }
    fn search_pid(&mut self, name: &str) -> Option<Vec<u32>> {
        let system = if let Ok(o) = self.system.lock() {
            o
        } else {
            return None;
        };
        let list = system.processes_by_name(name);
        let list = list.into_iter().map(|p| p.pid().as_u32()).collect();
        Some(list)
    }
    fn search_pid_exactly(&mut self, name: &str) -> Option<u32> {
        let system = if let Ok(o) = self.system.lock() {
            o
        } else {
            return None;
        };
        let list = system.processes_by_exact_name(name);
        // 优先返回pid更小的那个
        list.into_iter().map(|p| p.pid().as_u32()).min()
    }
}

fn update_all(system: &mut System) {
    system.refresh_processes_specifics(PROCESS_REFRESH!());
}

fn update_spec(system: &mut System, pid: Pid) {
    system.refresh_process_specifics(pid, PROCESS_REFRESH!());
}
