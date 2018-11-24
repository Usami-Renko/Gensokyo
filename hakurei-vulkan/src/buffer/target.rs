
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use memory::{ HaMemoryType, MemoryDstEntity };
use buffer::error::BufferError;

use types::{ vkbytes, vkuint };
use std::ptr;

pub struct HaBuffer {

    pub(crate) handle: vk::Buffer,
    requirement: vk::MemoryRequirements,
}

impl HaBuffer {

    fn new(device: &HaDevice, handle: vk::Buffer) -> HaBuffer {

        let requirement = unsafe {
            device.handle.get_buffer_memory_requirements(handle)
        };

        HaBuffer {
            handle, requirement,
        }
    }

    pub fn cleanup(&self, device: &HaDevice) {

        unsafe {
            device.handle.destroy_buffer(self.handle, None);
        }
    }
}

impl MemoryDstEntity for HaBuffer {

    fn type_bytes(&self) -> vkuint {
        self.requirement.memory_type_bits
    }

    fn aligment_size(&self) -> vkbytes {

        use utils::memory::bind_to_alignment;
        bind_to_alignment(self.requirement.size, self.requirement.alignment)
    }
}

#[derive(Debug, Clone, Default)]
pub struct BufferDescInfo {

    flags: vk::BufferCreateFlags,
    usage: vk::BufferUsageFlags,
    estimate_size: vkbytes,
}

impl BufferDescInfo {

    pub fn new(estimate_size: vkbytes, usage: vk::BufferUsageFlags) -> BufferDescInfo {

        BufferDescInfo {
            estimate_size, usage,
            ..Default::default()
        }
    }

    /// Generate a vk::buffer object.
    ///
    /// If the buffer is accessed by one queue family, set sharing_queue_families to None,
    /// or set it the queue family indices to share accessing.
    pub fn build(&self, device: &HaDevice, storage: BufferStorageType, sharing_queue_families: Option<Vec<vkuint>>) -> Result<HaBuffer, BufferError> {

        let (sharing_mode, indices) = match sharing_queue_families {
            | Some(families) => (vk::SharingMode::CONCURRENT, families),
            | None           => (vk::SharingMode::EXCLUSIVE, vec![]),
        };

        let create_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BUFFER_CREATE_INFO,
            p_next: ptr::null(),
            // TODO: Add configuration for BufferCreateFlag.
            flags: self.flags,
            size : self.estimate_size,
            usage: complement_buffer_usage(self.usage, storage),
            sharing_mode,
            queue_family_index_count: indices.len() as vkuint,
            p_queue_family_indices  : indices.as_ptr(),
        };

        let handle = unsafe {
            device.handle.create_buffer(&create_info, None)
                .or(Err(BufferError::BufferCreationError))?
        };

        let buffer = HaBuffer::new(device, handle);
        Ok(buffer)
    }

    pub fn with_flag(&mut self, flags: vk::BufferCreateFlags) {
        self.flags = flags;
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BufferStorageType {
    Host, Cached, Device, Staging,
}

impl BufferStorageType {

    pub fn memory_type(&self) -> HaMemoryType {
        match self {
            | BufferStorageType::Host    => HaMemoryType::HostMemory,
            | BufferStorageType::Cached  => HaMemoryType::CachedMemory,
            | BufferStorageType::Device  => HaMemoryType::DeviceMemory,
            | BufferStorageType::Staging => HaMemoryType::StagingMemory,
        }
    }
}

fn complement_buffer_usage(origin: vk::BufferUsageFlags, storage_type: BufferStorageType) -> vk::BufferUsageFlags {

    match storage_type {
        // No other specific flag is needed for Host Buffer.
        | BufferStorageType::Host    => origin,
        // Cached Buffer always need to be transfer dst.
        | BufferStorageType::Cached  => origin | vk::BufferUsageFlags::TRANSFER_DST,
        // Device Buffer always need to be transfer dst.
        | BufferStorageType::Device  => origin | vk::BufferUsageFlags::TRANSFER_DST,
        // Staging Buffer always need to be transfer src.
        | BufferStorageType::Staging => origin | vk::BufferUsageFlags::TRANSFER_SRC,
    }
}
