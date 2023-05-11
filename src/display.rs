use crate::{LimitArgs, Limiter, Task};
use std::fmt::{self, Display};
use sysinfo::{ProcessExt, SystemExt};

impl<'a: 'b, 'b> Display for Limiter<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "System: {}",
            self.system.name().unwrap_or(String::from("None"))
        )?;
        writeln!(f, "Tasks:")?;
        for task in self.tasks.values() {
            writeln!(f, "--------")?;
            writeln!(f, "{}", task)?;
            writeln!(f, "--------")?;
        }
        Ok(())
    }
}

impl<'a> Display for Task<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "name: {}", self.process.name())?;
        writeln!(f, "pid: {}", self.process.pid())?;
        writeln!(f, "cmd: {:?}", self.process.cmd())?;
        writeln!(f, "path: {}", self.process.exe().display())?;
        writeln!(f, "status: {}", self.process.status())?;
        Ok(())
    }
}

impl<'a> Display for LimitArgs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "process pid: {}", self.process.pid())?;
        writeln!(f, "current usage: {}", self.current_usage)?;
        writeln!(f, "target uasge: {}", self.target_usage)?;
        writeln!(f, "time slice(cosnt): 150ms")?;
        Ok(())
    }
}
