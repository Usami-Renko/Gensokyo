
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;
use resources::buffer::HaBuffer;
use resources::buffer::{ DeviceBufferConfig, HostBufferConfig };
use resources::buffer::{ BufferCreateFlag, BufferUsageFlag };
use resources::error::BufferError;
use utility::marker::{ VulkanFlags, VulkanEnum };

use std::ptr;

pub trait BufferConfigModifiable {

    fn set_flags(&mut self, flags: &[BufferCreateFlag]);
    fn add_item(&mut self, estimate_size: vk::DeviceSize) -> usize;
}


impl BufferConfigModifiable for HostBufferConfig {

    fn set_flags(&mut self, flags: &[BufferCreateFlag]) {
        self.flags = flags.flags();
    }

    /// estimate_size is the size in bytes of the buffer to be created. size must be greater than 0.
    fn add_item(&mut self, estimate_size: vk::DeviceSize) -> usize {
        let item_index = self.items_size.len();
        self.total_size += estimate_size;
        self.items_size.push(estimate_size);

        item_index
    }
}

impl BufferConfigModifiable for DeviceBufferConfig {

    fn set_flags(&mut self, flags: &[BufferCreateFlag]) {
        self.flags = flags.flags();
    }

    /// estimate_size is the size in bytes of the buffer to be created. size must be greater than 0.
    fn add_item(&mut self, estimate_size: vk::DeviceSize) -> usize {
        let item_index = self.items_size.len();
        self.total_size += estimate_size;
        self.items_size.push(estimate_size);

        item_index
    }
}


pub(crate) trait BufferGenerator {

    fn flags(&self)      -> vk::BufferCreateFlags;
    fn usage(&self)      -> vk::BufferUsageFlags;
    fn total_size(&self) -> vk::DeviceSize;

    /// Generate a buffer object.
    ///
    /// If the buffer is accessed by one queue family, set sharing_queue_families to None,
    /// or set it the queue family indices to share accessing.
    fn generate(&self, device: &HaLogicalDevice, sharing_queue_families: Option<Vec<uint32_t>>) -> Result<HaBuffer, BufferError> {

        let (sharing_mode, indices) = match sharing_queue_families {
            | Some(families) => (vk::SharingMode::Concurrent, families),
            | None           => (vk::SharingMode::Exclusive, vec![]),
        };

        let create_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BufferCreateInfo,
            p_next: ptr::null(),
            flags : self.flags(),
            size  : self.total_size(),
            usage : self.usage(),
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
            handle,
            requirement,
        };
        Ok(buffer)
    }
}

impl BufferGenerator for HostBufferConfig {

    fn flags(&self) -> vk::BufferCreateFlags { self.flags }
    fn usage(&self) -> vk::BufferUsageFlags  { self.usage }
    fn total_size(&self) -> vk::DeviceSize { self.total_size }
}

impl BufferGenerator for DeviceBufferConfig {

    fn flags(&self) -> vk::BufferCreateFlags { self.flags }
    fn usage(&self) -> vk::BufferUsageFlags  {
        // Device Buffer always need to be transfer dst.
        self.usage | BufferUsageFlag::TransferDstBit.value()
    }
    fn total_size(&self) -> vk::DeviceSize { self.total_size }
}
