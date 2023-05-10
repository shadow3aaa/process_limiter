use crate::Limiter;

pub trait LimiterExt {
    pub fn new() -> Self;
}

impl LimiterExt for Limiter