use crate::{Limiter, Task};
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

impl<'b> Display for Task<'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "name: {}", self.process.name())?;
        writeln!(f, "pid: {}", self.process.pid())?;
        writeln!(f, "cmd: {:?}", self.process.cmd())?;
        writeln!(f, "path: {}", self.process.exe().display())?;
        writeln!(f, "status: {}", self.process.status())?;
        Ok(())
    }
}
