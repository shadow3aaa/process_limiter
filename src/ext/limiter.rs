use super::task::TaskExt;
use crate::{Limiter, Task};
use std::{
    sync::{Arc, Mutex},
};
use sysinfo::{
    Pid, ProcessRefreshKind, RefreshKind, System, SystemExt,
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
    fn spawn(&mut self, pid: Pid) -> Task;
    fn search_pid(name: &str) -> Option<Vec<Pid>>;
}

impl LimiterExt for Limiter {
    fn new() -> Self {
        Self::default()
    }
    fn spawn(&mut self, _pid: Pid) -> Task {
        todo!()
    }
    fn search_pid(_name: &str) -> Option<Vec<Pid>> {
        todo!()
    }
}

fn update_all(system: &mut System) {
    system.refresh_processes_specifics(PROCESS_REFRESH!());
}

fn update_spec(system: &mut System, pid: Pid) {
    system.refresh_process_specifics(pid, PROCESS_REFRESH!());
}
