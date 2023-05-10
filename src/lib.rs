mod ext;
mod construct;

pub use construct::{
    limiter::*,
    task::*,
};
pub use ext::{
    limiter::LimiterExt,
    task::TaskExt,
};
use sysinfo::{
    System,
    SystemExt,
    Process,
};

pub struct Limiter<'sys> {
    system : System,
    tasks : Vec<Task<'sys>>,
}

pub struct Task<'sys> {
    process : &'sys Process,
}