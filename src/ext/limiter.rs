use crate::{Limiter, Task, TaskExt};
use std::sync::{Arc, Mutex};
use sysinfo::{PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

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
        let mut system = if let Ok(o) = self.system.lock() {
            o
        } else {
            return None;
        };
        update_all(&mut system);
        let list = system.processes_by_name(name);
        let list = list.into_iter().map(|p| p.pid().as_u32()).collect();
        Some(list)
    }
    fn search_pid_exactly(&mut self, name: &str) -> Option<u32> {
        let mut system = if let Ok(o) = self.system.lock() {
            o
        } else {
            return None;
        };
        update_all(&mut system);
        let list = system.processes_by_exact_name(name);
        // Preference is given to returning the one with the smaller PID
        list.into_iter().map(|p| p.pid().as_u32()).min()
    }
}

fn update_all(system: &mut System) {
    system.refresh_processes_specifics(PROCESS_REFRESH!());
}
