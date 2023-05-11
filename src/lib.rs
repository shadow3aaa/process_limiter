mod display;
mod ext;

pub use display::*;
pub use ext::{limiter::*, task::*};
use std::{collections::HashMap, marker::PhantomData};
use sysinfo::{Process, System};

#[derive(Debug)]
pub struct Limiter<'a: 'b, 'b> {
    system: System,
    tasks: HashMap<u32, Task<'b>>,
    _marker: PhantomData<&'a ()>,
}

#[derive(Debug)]
pub struct Task<'b> {
    process: &'b Process,
}
