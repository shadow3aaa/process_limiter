use super::task::TaskExt;
use crate::{Limiter, Task, Update};
use std::{
    collections::HashMap,
    marker::PhantomData,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
};
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
}

impl<'a: 'b, 'b> Default for Limiter<'a, 'b> {
    fn default() -> Self {
        let system = Arc::new(Mutex::new(System::new_with_specifics(
            RefreshKind::new().with_processes(PROCESS_REFRESH!()),
        )));

        let (s, r) = channel();
        let updater_thread = thread::spawn(move || {
            let system = system.clone();
            loop {
                let up_req = r.recv().unwrap();
                let system_up = &mut *system.lock().unwrap();
                match up_req {
                    Update::All => {
                        update_all(system_up);
                    }
                    Update::Spec(pid) => {
                        update_spec(system_up, pid);
                    }
                }
            }
        });
        Self {
            system,
            sender_orginal: s,
            updater_thread,
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
        let mut system = self.system.lock().unwrap();
        // Get the process
        let process = match by {
            CreatTaskBy::Pid(p) => system.process(Pid::from_u32(p))?,
            CreatTaskBy::Name(s) => system.processes_by_name(&s).next()?,
            CreatTaskBy::ExactName(s) => system.processes_by_exact_name(&s).next()?,
        };

        // Save the Task and return a ref
        let pid = process.pid();
        self.tasks.insert(pid, Task::new(process));
        self.tasks.get_mut(&pid)
    }
}

fn update_all(system: &mut System) {
    system.refresh_processes_specifics(PROCESS_REFRESH!());
}

fn update_spec(system: &mut System, pid: Pid) {
    system.refresh_process_specifics(pid, PROCESS_REFRESH!());
}
