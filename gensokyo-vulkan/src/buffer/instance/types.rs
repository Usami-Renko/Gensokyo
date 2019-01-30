
use ash::vk;

use crate::core::GsDevice;
use crate::memory::types::GsMemoryType;
use crate::types::vkbytes;

pub trait BufferCIAbstract<I>: Sized {
    const VK_FLAG: vk::BufferUsageFlags;

    fn check_storage_validity(memory_type: GsMemoryType) -> bool {
        check_buffer_usage(memory_type, Self::VK_FLAG)
    }

    fn estimate_size(&self) -> vkbytes;

    fn into_index(self) -> I;

    fn check_limits(&mut self, _device: &GsDevice) {
        // Default implementation is empty.
    }
}

fn check_buffer_usage(memory_type: GsMemoryType, buffer_flag: vk::BufferUsageFlags) -> bool {

    match memory_type {
        | GsMemoryType::HostMemory => {
            [
                vk::BufferUsageFlags::VERTEX_BUFFER,
                vk::BufferUsageFlags::INDEX_BUFFER,
                vk::BufferUsageFlags::UNIFORM_BUFFER,
            ].contains(&buffer_flag)
        },
        | GsMemoryType::CachedMemory  => {
            [
                vk::BufferUsageFlags::VERTEX_BUFFER,
                vk::BufferUsageFlags::INDEX_BUFFER,
            ].contains(&buffer_flag)
        },
        | GsMemoryType::DeviceMemory  => {
            [
                vk::BufferUsageFlags::VERTEX_BUFFER,
                vk::BufferUsageFlags::INDEX_BUFFER,
            ].contains(&buffer_flag)
        },
        | GsMemoryType::StagingMemory => {
            [
                vk::BufferUsageFlags::VERTEX_BUFFER,
                vk::BufferUsageFlags::INDEX_BUFFER,
                vk::BufferUsageFlags::UNIFORM_BUFFER,
                vk::BufferUsageFlags::TRANSFER_SRC,
            ].contains(&buffer_flag)
        },
    }
}
