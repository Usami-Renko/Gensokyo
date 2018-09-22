
use ash::vk;

use resources::buffer::{ DeviceBufferUsage, HostBufferUsage };
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

    pub(crate) fn to_host(&self) -> HostBufferConfig {
        HostBufferConfig {
            usage: vk::BufferUsageFlags::empty(),
            flags: vk::BufferCreateFlags::empty(),

            total_size: self.total_size,
            items_size: self.items_size.clone(),
        }
    }
}
