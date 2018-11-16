
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::buffer::target::{ HaBuffer, BufferStorageType };
use resources::buffer::item::BufferItem;
use resources::buffer::flag::{ BufferCreateFlag, BufferUsageFlag };
use resources::error::BufferError;

use utils::types::{ vkint, vkMemorySize };
use utils::marker::{ VulkanFlags, VulkanEnum };

use std::ptr;

// TODO: Make this to struct.
pub trait BufferBlockInfo {

    fn create_flags(&self) -> &[BufferCreateFlag];
    fn usage_flags(&self) -> &[BufferUsageFlag];
    fn estimate_size(&self) -> vkMemorySize;

    /// Generate a vk::buffer object.
    ///
    /// If the buffer is accessed by one queue family, set sharing_queue_families to None,
    /// or set it the queue family indices to share accessing.
    fn build(&self, device: &HaDevice, sharing_queue_families: Option<Vec<vkint>>, storage_type: BufferStorageType) -> Result<HaBuffer, BufferError> {

        let (sharing_mode, indices) = match sharing_queue_families {
            | Some(families) => (vk::SharingMode::Concurrent, families),
            | None           => (vk::SharingMode::Exclusive, vec![]),
        };

        let create_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BufferCreateInfo,
            p_next: ptr::null(),
            flags: self.create_flags().flags(),
            size : self.estimate_size(),
            usage: complement_buffer_usage(self.usage_flags().flags(), storage_type),
            sharing_mode,
            queue_family_index_count: indices.len() as vkint,
            p_queue_family_indices  : indices.as_ptr(),
        };

        let handle = unsafe {
            device.handle.create_buffer(&create_info, None)
                .or(Err(BufferError::BufferCreationError))?
        };

        let buffer = HaBuffer::new(device, handle);

        Ok(buffer)
    }
}

pub trait BufferBlockEntity: BufferCopiable {

    fn item(&self) -> &BufferItem;
    fn offset(&self, sub_index: usize) -> vkMemorySize;
}

pub trait BufferCopiable {

    fn copy_info(&self) -> BufferCopyInfo;
}

pub trait BufferHandleEntity {

    fn handle(&self) -> vk::Buffer;
}

pub struct BufferCopyInfo {

    /// `handle` the handle of buffer whose data is copied from or copy to.
    pub(crate) handle: vk::Buffer,
    /// `offset` the starting offset in bytes from the start of source or destination buffer.
    pub(crate) offset: vkMemorySize,
    /// If this is the buffer for data source, `size` is the number of bytes to copy.
    ///
    /// If this is the buffer for data destination, `size` will be ignored.
    pub(crate) size: vkMemorySize,
}

impl BufferCopyInfo {

    pub fn new(buffer: &impl BufferHandleEntity, offset: vkMemorySize, size: vkMemorySize) -> BufferCopyInfo {

        BufferCopyInfo {
            handle: buffer.handle(),
            offset,
            size,
        }
    }
}

fn complement_buffer_usage(origin: vk::BufferUsageFlags, storage_type: BufferStorageType) -> vk::BufferUsageFlags {

    match storage_type {
        // No other specific flag is needed for Host Buffer.
        | BufferStorageType::Host    => origin,
        // Cached Buffer always need to be transfer dst.
        | BufferStorageType::Cached  => origin | BufferUsageFlag::TransferDstBit.value(),
        // Device Buffer always need to be transfer dst.
        | BufferStorageType::Device  => origin | BufferUsageFlag::TransferDstBit.value(),
        // Staging Buffer always need to be transfer src.
        | BufferStorageType::Staging => origin | BufferUsageFlag::TransferSrcBit.value(),
    }
}
