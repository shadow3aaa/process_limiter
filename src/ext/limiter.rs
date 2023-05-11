use super::task::TaskExt;
use crate::{Limiter, Task};
use std::{collections::HashMap, marker::PhantomData};
use sysinfo::{Pid, PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

// What the Process needs to be refreshed
macro_rules! PROCESS_REFRESH {
    () => {
        ProcessRefreshKind::new().with_cpu().with_user()
    };
}

// Create credentials for the restricted task
pub enum CreatTaskBy {
    Pid(u32),
    Name(String),
    ExactName(String),
}

pub trait LimiterExt<'a: 'b, 'b> {
    // Constructor
    fn new() -> Self;
    // Create a Task, one for each Process
    // Task is the type of actual operation restriction process, and Limiter manages Tasks
    fn new_task(&'a mut self, by: CreatTaskBy) -> Option<&'b mut Task<'b>>;
    // Refresh all tasks run by the device
    fn refresh_processes(&'a mut self);
    // Refreshes the specified Task information
    // True: task exist
    // False: task doesn’t exist
    fn refresh_task(&'a mut self, pid: Pid) -> bool;
}

impl<'a: 'b, 'b> Default for Limiter<'a, 'b> {
    fn default() -> Self {
        Self {
            system: System::new_with_specifics(
                RefreshKind::new().with_processes(PROCESS_REFRESH!()),
            ),
            tasks: HashMap::<Pid, Task<'b>>::new(),
            _marker: PhantomData,
        }
    }
}

impl<'a: 'b, 'b> LimiterExt<'a, 'b> for Limiter<'a, 'b> {
    fn new() -> Self {
        Self::default()
    }
    fn new_task(&'a mut self, by: CreatTaskBy) -> Option<&'b mut Task<'b>> {
        // Get the process
        let process = match by {
            CreatTaskBy::Pid(p) => self.system.process(Pid::from_u32(p))?,
            CreatTaskBy::Name(s) => self.system.processes_by_name(&s).next()?,
            CreatTaskBy::ExactName(s) => self.system.processes_by_exact_name(&s).next()?,
        };

        // Save the Task and return a ref
        let pid = process.pid();
        self.tasks.insert(pid, Task::new(process));
        self.tasks.get_mut(&pid)
    }
    fn refresh_processes(&'a mut self) {
        self.system.refresh_processes_specifics(PROCESS_REFRESH!())
    }
    fn refresh_task(&'a mut self, pid: Pid) -> bool {
        self.system
            .refresh_process_specifics(pid, PROCESS_REFRESH!())
    }
}
