use crate::{
    Limiter,
    Task
};
use sysinfo::{
    System,
    SystemExt
};

impl<'sys> Default for Limiter<'sys> {
    fn default() -> Self {
        Self {
            system : System::new_all(),
            tasks : Vec::<Task<'sys>>::new()
        }
    }
}