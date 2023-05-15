use crate::{Limiter, Task};
use std::fmt::{self, Display};
use sysinfo::ProcessExt;

impl<'a: 'b, 'b> Display for Limiter<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
