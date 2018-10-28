
use ash::vk;
use ash::vk::uint32_t;

use resources::buffer::{ BufferItem, BufferUsageFlag, BufferCreateFlag };
use resources::buffer::{ BufferBlockInfo, BufferBlockEntity };
use resources::buffer::{ BufferCopiable, BufferCopyInfo };
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

    element_size: vk::DeviceSize,
}

impl UniformBlockInfo {

    pub fn new(binding: uint32_t, count: uint32_t, element_size: vk::DeviceSize) -> UniformBlockInfo {
        UniformBlockInfo {
            binding, count,
            element_size,
            flags: vk::BufferCreateFlags::empty(),
        }
    }

    pub fn add_flag(&mut self, flag: BufferCreateFlag) {
        self.flags |= flag.value()
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
        self.element_size * (self.count as vk::DeviceSize)
    }
}

impl BufferInfosAllocatable for UniformBlockInfo {}


#[derive(Default)]
pub struct HaUniformBlock {

    binding: uint32_t,
    count  : uint32_t,

    item: BufferItem,
    element_size: vk::DeviceSize,
}

impl HaUniformBlock {

    pub fn uninitialize() -> HaUniformBlock {
        HaUniformBlock::default()
    }

    pub(crate) fn from(info: &UniformBlockInfo, item: BufferItem) -> HaUniformBlock {

        HaUniformBlock {
            binding: info.binding,
            count  : info.count,
            element_size: info.element_size,
            item,
        }
    }
}

impl DescriptorBufferBindableTarget for HaUniformBlock {

    fn binding_info(&self, sub_block_indices: Option<Vec<uint32_t>>) -> Result<DescriptorBufferBindingInfo, DescriptorError> {

        let info = DescriptorBufferBindingInfo {
            typ: BufferDescriptorType::UniformBuffer,
            binding: self.binding,
            count  : self.count,
            element_indices: sub_block_indices.unwrap_or(vec![0]),
            element_size: self.element_size,
            buffer: self.item.clone(),
        };

        Ok(info)
    }
}

impl BufferBlockEntity for HaUniformBlock {

    fn get_buffer_item(&self) -> &BufferItem {
        &self.item
    }

    fn offset(&self, sub_index: usize) -> vk::DeviceSize {
        self.element_size * (sub_index as vk::DeviceSize)
    }
}

impl BufferCopiable for HaUniformBlock {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo {
            handle: self.item.handle,
            offset: 0,
            size  : self.item.size,
        }
    }
}
