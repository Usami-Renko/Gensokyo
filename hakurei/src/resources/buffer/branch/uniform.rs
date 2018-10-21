
use ash::vk;
use ash::vk::uint32_t;

use resources::buffer::{ BufferSubItem, BufferUsageFlag };
use resources::buffer::{ BufferBlockInfo, BufferBlockEntity };
use resources::allocator::BufferInfosAllocatable;
use resources::descriptor::{ DescriptorBufferBindingInfo, DescriptorBufferBindableTarget };
use resources::descriptor::BufferDescriptorType;
use resources::error::DescriptorError;

use utility::marker::VulkanEnum;

#[derive(Debug, Clone)]
pub struct UniformBlockInfo {

    binding: uint32_t,
    count  : uint32_t,

    flags: vk::BufferCreateFlags,

    estimate_size: vk::DeviceSize,
}

impl UniformBlockInfo {

    pub fn new(binding: uint32_t, count: uint32_t, estimate_size: vk::DeviceSize) -> UniformBlockInfo {
        UniformBlockInfo {
            binding, count, estimate_size,
            flags: vk::BufferCreateFlags::empty(),
        }
    }
}

impl BufferBlockInfo for UniformBlockInfo {

    fn flags(&self) -> vk::BufferCreateFlags {
        self.flags
    }

    fn usage(&self) -> vk::BufferUsageFlags {
        BufferUsageFlag::UniformBufferBit.value()
    }

    fn total_size(&self) -> vk::DeviceSize {
        self.estimate_size
    }
}

impl BufferInfosAllocatable for UniformBlockInfo {}

pub struct HaUniformBlock {

    binding: uint32_t,
    count  : uint32_t,

    item: BufferSubItem,
}

impl HaUniformBlock {

    pub fn uninitialize() -> HaUniformBlock {
        HaUniformBlock {
            binding: 0,
            count  : 0,
            item: BufferSubItem::unset(),
        }
    }

    pub(crate) fn from(info: &UniformBlockInfo, item: BufferSubItem) -> HaUniformBlock {

        HaUniformBlock {
            binding: info.binding,
            count  : info.count,
            item,
        }
    }
}

impl DescriptorBufferBindableTarget for HaUniformBlock {

    fn binding_info(&self) -> Result<DescriptorBufferBindingInfo, DescriptorError> {

        let info = DescriptorBufferBindingInfo {
            type_  : BufferDescriptorType::UniformBuffer,
            binding: self.binding,
            count  : self.count,
            element_size: self.item.size,
            buffer: self.item.clone(),
        };

        Ok(info)
    }
}

impl BufferBlockEntity for HaUniformBlock {

    fn get_buffer_item(&self) -> &BufferSubItem {
        &self.item
    }
}
