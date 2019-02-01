
use ash::vk;

use crate::core::GsDevice;
use crate::buffer::entity::BufferBlock;
use crate::buffer::instance::types::BufferCIApi;
use crate::buffer::traits::{ BufferInstance, BufferCopiable, BufferCopyInfo };

use crate::descriptor::DescriptorBufferBindableTarget;
use crate::descriptor::{ DescriptorBindingContent, DescriptorBufferBindingInfo };
use crate::descriptor::{ GsDescriptorType, BufferDescriptorType };

use crate::memory::transfer::MemoryDataDelegate;
use crate::memory::MemoryWritePtr;

use crate::error::VkResult;
use crate::types::{ vkuint, vkbytes };
use crate::utils::memory::bound_to_alignment;

#[derive(Debug, Clone)]
pub struct UniformBufferCI {

    usage: UniformUsage,
    binding: DescriptorBindingContent,
    /// the actual data size of each element.
    element_size: vkbytes,
    /// the minimum uniform buffer offset alignment required by Vulkan.
    alignment: vkbytes,
}

#[derive(Debug, Clone)]
enum UniformUsage {
    Common,
    Dynamic { slice_count: vkuint, slice_size: vkbytes },
}

impl GsUniformBuffer {

    /// Prepare to create a Common Uniform Buffer.
    pub fn new(binding: vkuint, descriptor_count: vkuint, element_size: vkbytes) -> UniformBufferCI {

        UniformBufferCI {
            usage: UniformUsage::Common,
            binding: DescriptorBindingContent {
                binding,
                count: descriptor_count,
                descriptor_type: GsDescriptorType::Buffer(BufferDescriptorType::UniformBuffer),
            },
            element_size,
            alignment: 0, // alignment will be set when add it to allocator.
        }
    }

    /// Prepare to create a Dynamic Uniform Buffer.
    pub fn new_dyn(binding: vkuint, descriptor_count: vkuint, slice_size: vkbytes, slice_count: usize) -> UniformBufferCI {

        UniformBufferCI {
            usage: UniformUsage::Dynamic {
                slice_count: slice_count as vkuint,
                slice_size,
            },
            binding: DescriptorBindingContent {
                binding,
                count: descriptor_count,
                descriptor_type: GsDescriptorType::Buffer(BufferDescriptorType::DynamicUniformBuffer),
            },
            element_size: slice_size * (slice_count as vkbytes),
            alignment: 0, // alignment will be set when add it to allocator.
        }
    }
}

impl UniformBufferCI {

    fn set_alignment(&mut self, device: &GsDevice) {
        // query alignment from Vulkan.
        self.alignment = device.phys.limits().min_uniform_buffer_offset_alignment;
    }
}

impl BufferCIApi for UniformBufferCI {
    type IConveyor = IUniform;

    const VK_FLAG: vk::BufferUsageFlags = vk::BufferUsageFlags::UNIFORM_BUFFER;

    fn estimate_size(&self) -> vkbytes {

        match self.usage {
            | UniformUsage::Common => {
                (self.binding.count as vkbytes) * self.element_size
            },
            | UniformUsage::Dynamic { slice_count, slice_size } => {
                bound_to_alignment(slice_size, self.alignment) * (slice_count as vkbytes) * (self.binding.count as vkbytes)
            },
        }
    }

    fn into_index(self) -> IUniform {
        
        IUniform {
            usage: self.usage,
            binding: self.binding,
            element_size: self.element_size,
            alignment: self.alignment,
        }
    }

    // Handle uniform buffer particularly.
    fn check_limits(&mut self, device: &GsDevice) {
        self.set_alignment(device);
    }
}

pub struct IUniform {

    usage: UniformUsage,
    binding: DescriptorBindingContent,
    alignment: vkbytes,
    element_size: vkbytes,
}


pub struct GsUniformBuffer {

    iuniform: IUniform,

    block: BufferBlock,
    repository_index: usize,
}


impl BufferInstance for GsUniformBuffer {
    type InfoType = IUniform;

    fn build(block: BufferBlock, info: Self::InfoType, repository_index: usize) -> Self {

        GsUniformBuffer {
            iuniform: info,
            block, repository_index,
        }
    }

    fn acquire_write_ptr(&self, agency: &mut Box<dyn MemoryDataDelegate>) -> VkResult<MemoryWritePtr> {
        agency.acquire_write_ptr(&self.block, self.repository_index)
    }
}

impl DescriptorBufferBindableTarget for GsUniformBuffer {

    fn binding_info(&self, sub_block_indices: Option<Vec<vkuint>>) -> DescriptorBufferBindingInfo {

        DescriptorBufferBindingInfo {
            content: self.iuniform.binding.clone(),
            element_indices: sub_block_indices.unwrap_or(vec![0]),
            buffer_handle: self.block.handle,
            element_size: self.element_size(),
        }
    }
}

impl BufferCopiable for GsUniformBuffer {

    fn copy_whole(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.block, 0, self.block.size)
    }
}

impl GsUniformBuffer {

    /// Return the min uniform offset alignment query from Vulkan.
    ///
    /// This value is only meaningful to dynamic uniform buffer.
    pub fn require_dynamic_alignment(&self) -> vkbytes {
        self.iuniform.alignment
    }

    /// For common uniform buffer, this func just return the whole size of this uniform buffer.
    ///
    /// For dynamic uniform buffer, this func return the aligned size of each element.
    pub fn aligned_size(&self) -> vkbytes {

        match self.iuniform.usage {
            | UniformUsage::Common => self.block.size,
            | UniformUsage::Dynamic { slice_size, .. } => {
                bound_to_alignment(slice_size, self.iuniform.alignment)
            }
        }
    }

    fn element_size(&self) -> vkbytes {

        match self.iuniform.usage {
            | UniformUsage::Common => self.iuniform.element_size,
            | UniformUsage::Dynamic { slice_size, .. } => {
                bound_to_alignment(slice_size, self.iuniform.alignment)
            }
        }
    }
}
