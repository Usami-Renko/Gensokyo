
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::memory::{ HaMemoryType, MemoryDstEntity };
use utils::types::{ vkint, vkMemorySize };

pub struct HaBuffer {

    pub handle: vk::Buffer,
    requirement: vk::MemoryRequirements,
}

impl HaBuffer {

    pub(crate) fn new(device: &HaDevice, handle: vk::Buffer) -> HaBuffer {

        let requirement = device.handle.get_buffer_memory_requirements(handle);

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

    fn type_bytes(&self) -> vkint {
        self.requirement.memory_type_bits
    }

    fn aligment_size(&self) -> vkMemorySize {

        use utils::memory::bind_to_alignment;
        bind_to_alignment(self.requirement.size, self.requirement.alignment)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BufferStorageType {
    Host,
    Cached,
    Device,
    Staging,
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