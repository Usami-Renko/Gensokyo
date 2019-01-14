
use ash::vk;

use crate::core::physical::GsPhysicalDevice;

use crate::buffer::entity::BufferBlock;
use crate::buffer::instance::types::BufferInfoAbstract;
use crate::buffer::traits::{ BufferInstance, BufferCopiable, BufferCopyInfo };

use crate::descriptor::DescriptorBufferBindableTarget;
use crate::descriptor::{ DescriptorBindingContent, DescriptorBufferBindingInfo };
use crate::descriptor::{ GsDescriptorType, BufferDescriptorType };

use crate::memory::transfer::MemoryDataDelegate;
use crate::memory::{ MemoryWritePtr, MemoryError };

use crate::types::{ vkuint, vkbytes };
use crate::utils::memory::bound_to_alignment;

#[derive(Debug, Clone)]
pub struct GsBufUniformInfo {

    usage: UniformUsage,
    binding: DescriptorBindingContent,
    element_size: vkbytes,
}

#[derive(Debug, Clone)]
enum UniformUsage {
    Common,
    Dynamic { slice_count: vkuint, slice_size: vkbytes, alignment: vkbytes },
}

impl GsBufUniformInfo {

    /// Prepare to create a Common Uniform Buffer.
    pub fn new(binding: vkuint, descriptor_count: vkuint, element_size: vkbytes) -> GsBufUniformInfo {

        GsBufUniformInfo {
            usage: UniformUsage::Common,
            binding: DescriptorBindingContent {
                binding,
                count: descriptor_count,
                descriptor_type: GsDescriptorType::Buffer(BufferDescriptorType::UniformBuffer),
            },
            element_size,
        }
    }

    /// Prepare to create a Dynamic Uniform Buffer.
    pub fn new_dyn(binding: vkuint, descriptor_count: vkuint, slice_size: vkbytes, slice_count: usize) -> GsBufUniformInfo {

        GsBufUniformInfo {
            usage: UniformUsage::Dynamic {
                slice_count: slice_count as vkuint,
                slice_size,
                alignment: 0, // alignment will be set when add it to allocator.
            },
            binding: DescriptorBindingContent {
                binding,
                count: descriptor_count,
                descriptor_type: GsDescriptorType::Buffer(BufferDescriptorType::UniformBufferDynamic),
            },
            element_size: slice_size * (slice_count as vkbytes),
        }
    }
}

impl BufferInfoAbstract<IUniform> for GsBufUniformInfo {
    const VK_FLAG: vk::BufferUsageFlags = vk::BufferUsageFlags::UNIFORM_BUFFER;

    fn estimate_size(&self) -> vkbytes {

        match self.usage {
            | UniformUsage::Common => {
                (self.binding.count as vkbytes) * self.element_size
            },
            | UniformUsage::Dynamic { slice_count, slice_size, alignment } => {
                bound_to_alignment(slice_size, alignment) * (slice_count as vkbytes) * (self.binding.count as vkbytes)
            },
        }
    }

    fn into_index(self) -> IUniform {
        
        IUniform {
            usage: self.usage,
            binding: self.binding,
            element_size: self.element_size,
        }
    }

    // Handle uniform buffer particularly.
    fn check_limits(&mut self, physical: &GsPhysicalDevice) {
        self.usage.set_alignment(physical);
    }
}

pub struct IUniform {

    usage: UniformUsage,
    binding: DescriptorBindingContent,
    element_size: vkbytes,
}


pub struct GsUniformBuffer {

    usage: UniformUsage,
    binding: DescriptorBindingContent,
    element_size: vkbytes,

    block: BufferBlock,
    repository_index: usize,
}


impl BufferInstance for GsUniformBuffer {
    type InfoType = IUniform;

    fn new(block: BufferBlock, info: Self::InfoType, repository_index: usize) -> Self {

        GsUniformBuffer {
            usage: info.usage,
            binding: info.binding,
            element_size: info.element_size,
            block, repository_index,
        }
    }

    fn acquire_write_ptr(&self, agency: &mut Box<dyn MemoryDataDelegate>) -> Result<MemoryWritePtr, MemoryError> {
        agency.acquire_write_ptr(&self.block, self.repository_index)
    }
}

impl DescriptorBufferBindableTarget for GsUniformBuffer {

    fn binding_info(&self, sub_block_indices: Option<Vec<vkuint>>) -> DescriptorBufferBindingInfo {

        DescriptorBufferBindingInfo {
            content: self.binding.clone(),
            element_indices: sub_block_indices.unwrap_or(vec![0]),
            buffer_handle: self.block.handle,
            element_size: match self.usage {
                | UniformUsage::Common => self.element_size,
                | UniformUsage::Dynamic { slice_size, alignment, .. } => {
                    // bind_to_alignment(slice_size, alignment) * (slice_count as vkbytes)
                    bound_to_alignment(slice_size, alignment)
                },
            },
        }
    }
}

impl BufferCopiable for GsUniformBuffer {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.block, 0, self.block.size)
    }
}

impl GsUniformBuffer {

    pub fn dyn_alignment(&self) -> Option<vkbytes> {
        match self.usage {
            | UniformUsage::Common => None,
            | UniformUsage::Dynamic { alignment, .. } => Some(alignment)
        }
    }
}


impl UniformUsage {

    pub(crate) fn set_alignment(&mut self, physical: &GsPhysicalDevice) {
        match self {
            | UniformUsage::Common => {},
            | UniformUsage::Dynamic { slice_count: _, slice_size: _, ref mut alignment } => {
                *alignment = physical.properties.limits().min_uniform_buffer_offset_alignment;
            },
        }
    }
}
