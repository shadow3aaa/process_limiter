use super::task::TaskExt;
use crate::{Limiter, Task};
use std::{collections::HashMap, marker::PhantomData};
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

pub enum CreatTaskBy {
    Pid(u32),
    Name(String),
    ExactName(String),
}

pub trait LimiterExt<'a: 'b, 'b> {
    fn new() -> Self;
    fn new_task(&'a mut self, by: CreatTaskBy) -> Option<&'b mut Task<'b>>;
}

impl<'a: 'b, 'b> Default for Limiter<'a, 'b> {
    fn default() -> Self {
        Self {
            system: System::new_all(),
            tasks: HashMap::<u32, Task<'b>>::new(),
            _marker: PhantomData,
        }
    }
}

impl<'a: 'b, 'b> LimiterExt<'a, 'b> for Limiter<'a, 'b> {
    fn new() -> Self {
        Self::default()
    }
    fn new_task(&'a mut self, by: CreatTaskBy) -> Option<&'b mut Task<'b>> {
        let process = match by {
            CreatTaskBy::Pid(p) => self.system.process(Pid::from_u32(p))?,
            CreatTaskBy::Name(s) => self.system.processes_by_name(&s).next()?,
            CreatTaskBy::ExactName(s) => self.system.processes_by_exact_name(&s).next()?,
        };
        let pid = process.pid().as_u32();
        self.tasks.insert(pid, Task::new(process));
        self.tasks.get_mut(&pid)
    }
}
