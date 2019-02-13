
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::GsDevice;

use crate::buffer::allocator::types::BufferMemoryTypeAbs;
use crate::memory::MemoryDstEntity;

use crate::error::{ VkResult, VkError };
use crate::types::{ vkbytes, vkuint };

use std::ptr;

pub struct GsBuffer {

    pub(crate) handle: vk::Buffer,
    requirement: vk::MemoryRequirements,
}

impl GsBuffer {

    pub fn new(estimate_size: vkbytes, usage: vk::BufferUsageFlags) -> BufferCI {

        BufferCI {
            estimate_size, usage,
            flags: vk::BufferCreateFlags::empty(),
            sharing_queue_families: None,
        }
    }

    #[inline(always)]
    fn build(device: &GsDevice, buffer_ci: vk::BufferCreateInfo) -> VkResult<GsBuffer> {

        let handle = unsafe {
            device.logic.handle.create_buffer(&buffer_ci, None)
                .or(Err(VkError::create("vk::Buffer")))?
        };

        let requirement = unsafe {
            device.logic.handle.get_buffer_memory_requirements(handle)
        };

        let buffer = GsBuffer { handle, requirement };
        Ok(buffer)
    }

    /// Destroy vk::Buffer object in Vulkan.
    pub fn discard(&self, device: &GsDevice) {

        unsafe {
            device.logic.handle.destroy_buffer(self.handle, None);
        }
    }
}

impl MemoryDstEntity for GsBuffer {

    fn type_bytes(&self) -> vkuint {
        self.requirement.memory_type_bits
    }

    fn aligned_size(&self) -> vkbytes {

        use crate::utils::memory::bound_to_alignment;
        bound_to_alignment(self.requirement.size, self.requirement.alignment)
    }
}

#[derive(Debug, Clone, Default)]
pub struct BufferCI {

    flags: vk::BufferCreateFlags,
    usage: vk::BufferUsageFlags,
    estimate_size: vkbytes,

    sharing_queue_families: Option<Vec<vkuint>>,
}

impl BufferCI {

    // TODO: Add configuration for vk::BufferCreateFlags.
    pub fn with_flag(mut self, flags: vk::BufferCreateFlags) -> BufferCI {

        self.flags = flags;
        self
    }

    pub fn with_share_queue_families(mut self, families: Vec<vkuint>) -> BufferCI {

        self.sharing_queue_families = Some(families);
        self
    }

    /// Generate a vk::buffer object.
    ///
    /// If the buffer is accessed by one queue family, set sharing_queue_families to None,
    /// or set it the queue family indices to share accessing.
    pub fn build(&self, device: &GsDevice, memory_abs: impl BufferMemoryTypeAbs) -> VkResult<GsBuffer> {

        let mut buffer_ci = vk::BufferCreateInfo {
            s_type                   : vk::StructureType::BUFFER_CREATE_INFO,
            p_next                   : ptr::null(),
            flags                    : self.flags,
            size                     : self.estimate_size,
            usage                    : memory_abs.complement_usage(self.usage),
            sharing_mode             : vk::SharingMode::EXCLUSIVE,
            queue_family_index_count : 0,
            p_queue_family_indices   : ptr::null(),
        };

        if let Some(ref families) = self.sharing_queue_families {
            buffer_ci.sharing_mode = vk::SharingMode::CONCURRENT;
            buffer_ci.queue_family_index_count = families.len() as _;
            buffer_ci.p_queue_family_indices   = families.as_ptr();
        };

        GsBuffer::build(device, buffer_ci)
    }
}
