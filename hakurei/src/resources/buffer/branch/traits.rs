
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::buffer::{ HaBuffer, BufferSubItem, BufferUsageFlag };
use resources::allocator::BufferStorageType;
use resources::error::BufferError;
use utility::marker::VulkanEnum;

use std::ptr;

pub(crate) trait BufferBlockInfo {

    fn flags(&self) -> vk::BufferCreateFlags;
    fn usage(&self) -> vk::BufferUsageFlags;
    fn total_size(&self) -> vk::DeviceSize;

    /// Generate a vk::buffer object.
    ///
    /// If the buffer is accessed by one queue family, set sharing_queue_families to None,
    /// or set it the queue family indices to share accessing.
    fn build(&self, device: &HaDevice, sharing_queue_families: Option<Vec<uint32_t>>, storage_type: BufferStorageType) -> Result<HaBuffer, BufferError> {

        let (sharing_mode, indices) = match sharing_queue_families {
            | Some(families) => (vk::SharingMode::Concurrent, families),
            | None           => (vk::SharingMode::Exclusive, vec![]),
        };

        let create_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BufferCreateInfo,
            p_next: ptr::null(),
            flags: self.flags(),
            size : self.total_size(),
            usage: complement_buffer_usage(self.usage(), storage_type),
            sharing_mode,
            queue_family_index_count: indices.len() as uint32_t,
            p_queue_family_indices  : indices.as_ptr(),
        };

        let handle = unsafe {
            device.handle.create_buffer(&create_info, None)
                .or(Err(BufferError::BufferCreationError))?
        };

        let requirement = device.handle.get_buffer_memory_requirements(handle);

        let buffer = HaBuffer {
            handle, requirement,
        };
        Ok(buffer)
    }
}

pub trait BufferBlockEntity {

    fn get_buffer_item(&self) -> &BufferSubItem;
}

fn complement_buffer_usage(origin: vk::BufferUsageFlags, storage_type: BufferStorageType) -> vk::BufferUsageFlags {
    match storage_type {
        // No other specific flag is needed for Host Buffer
        | BufferStorageType::Host    => origin,
        // Cached Buffer always need to be transfer dst.
        | BufferStorageType::Cached  => origin | BufferUsageFlag::TransferDstBit.value(),
        // Device Buffer always need to be transfer dst.
        | BufferStorageType::Device  => origin | BufferUsageFlag::TransferDstBit.value(),
        // Staging Buffer always need to be transfer src.
        | BufferStorageType::Staging => origin | BufferUsageFlag::TransferSrcBit.value(),
    }
}
