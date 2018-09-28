
use ash::vk;

use resources::buffer::{ HostBufferUsage, CachedBufferUsage, DeviceBufferUsage, StagingBufferUsage };
use utility::marker::VulkanEnum;

#[derive(Debug, Clone)]
pub struct HostBufferConfig {

    pub(crate) usage: vk::BufferUsageFlags,
    pub(crate) flags: vk::BufferCreateFlags,

    pub(crate) total_size: vk::DeviceSize,
    pub(crate) items_size: Vec<vk::DeviceSize>,
}

impl HostBufferConfig {

    pub fn new(usage: HostBufferUsage) -> HostBufferConfig {
        HostBufferConfig {
            usage: usage.value(),
            flags: vk::BufferCreateFlags::empty(),
            total_size: 0,
            items_size: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct CachedBufferConfig {

    pub(crate) usage: vk::BufferUsageFlags,
    pub(crate) flags: vk::BufferCreateFlags,

    pub(crate) total_size: vk::DeviceSize,
    pub(crate) items_size: Vec<vk::DeviceSize>,
}

impl CachedBufferConfig {

    pub fn new(usage: CachedBufferUsage) -> CachedBufferConfig {
        CachedBufferConfig {
            usage: usage.value(),
            flags: vk::BufferCreateFlags::empty(),
            total_size: 0,
            items_size: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceBufferConfig {

    pub(crate) usage: vk::BufferUsageFlags,
    pub(crate) flags: vk::BufferCreateFlags,

    pub(crate) total_size: vk::DeviceSize,
    pub(crate) items_size: Vec<vk::DeviceSize>,
}

impl DeviceBufferConfig {

    pub fn new(usage: DeviceBufferUsage) -> DeviceBufferConfig {
        DeviceBufferConfig {
            usage: usage.value(),
            flags: vk::BufferCreateFlags::empty(),
            total_size: 0,
            items_size: vec![],
        }
    }
}

pub struct StagingBufferConfig {

    pub(crate) usage: vk::BufferUsageFlags,
    pub(crate) flags: vk::BufferCreateFlags,

    pub(crate) total_size: vk::DeviceSize,
    pub(crate) items_size: Vec<vk::DeviceSize>,
}

impl StagingBufferConfig {

    pub fn new(usage: StagingBufferUsage) -> StagingBufferConfig {
        StagingBufferConfig {
            usage: usage.value(),
            flags: vk::BufferCreateFlags::empty(),
            total_size: 0,
            items_size: vec![],
        }
    }
}
