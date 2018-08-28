
use ash::vk::uint64_t;

use std::time::Duration;

#[derive(Debug, Copy, Clone)]
pub enum TimePeriod {
    Immediate,
    Time(Duration),
    Infinte,
}

impl TimePeriod {

    pub fn vulkan_time(&self) -> uint64_t {
        match *self {
            | TimePeriod::Immediate => 0,
            | TimePeriod::Time(time) =>
                (time.subsec_nanos() as uint64_t) + time.as_secs() * 1_000_000_000,
            | TimePeriod::Infinte => uint64_t::max_value(),
        }
    }
}
