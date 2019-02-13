
use std::time::Duration;

use gsvk::types::vklint;

#[derive(Debug, Copy, Clone)]
pub enum TimePeriod {
    Immediate,
    Time(Duration),
    Infinite,
}

impl TimePeriod {

    pub fn vulkan_time(&self) -> vklint {
        match self {
            | TimePeriod::Immediate => 0,
            | TimePeriod::Time(time) =>
                (time.subsec_nanos() as vklint) + time.as_secs() * 1_000_000_000,
            | TimePeriod::Infinite => vklint::max_value(),
        }
    }
}
