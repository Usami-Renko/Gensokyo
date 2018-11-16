
use ash::vk;

use resources::descriptor::set::HaDescriptorSet;
use resources::descriptor::enums::HaDescriptorType;
use resources::buffer::BufferItem;
use resources::image::{ ImageViewItem, HaSampler, ImageLayout };

use utils::wrapper::VKWrapperInfo;
use utils::types::{ vkint, vkMemorySize };
use utils::marker::VulkanEnum;

use std::ptr;

pub type DescriptorWriteInfo = VKWrapperInfo<DescriptorWriteContent, vk::WriteDescriptorSet>;

#[derive(Debug, Clone)]
pub struct DescriptorBindingContent {

    /// the binding index used in shader for the descriptor.
    pub binding: vkint,
    /// the total element count of each descriptor.
    pub count: vkint,
    /// the type of descriptor.
    pub descriptor_type: HaDescriptorType,
}

pub trait DescriptorBindingInfo {

    fn binding_content(&self) -> &DescriptorBindingContent;
    fn write_set(&self, set: &HaDescriptorSet) -> DescriptorWriteInfo;
}

pub trait DescriptorWriteContent {}

pub trait DescriptorBufferBindableTarget {
    fn binding_info(&self, sub_block_indices: Option<Vec<vkint>>) -> DescriptorBufferBindingInfo;
}
pub trait DescriptorImageBindableTarget {
    fn binding_info(&self) -> DescriptorImageBindingInfo;
}

pub struct DescriptorBufferBindingInfo<'a> {

    pub content: DescriptorBindingContent,
    /// the element index of each descriptor to update.
    pub element_indices: Vec<vkint>,
    /// the size of each element of descriptor.
    pub element_size: vkMemorySize,
    /// the reference to buffer where the descriptor data stores.
    pub buffer: &'a BufferItem,
}

struct DescriptorWriteBufferContent(Vec<vk::DescriptorBufferInfo>);
impl DescriptorWriteContent for DescriptorWriteBufferContent {}

impl<'a> DescriptorBindingInfo for DescriptorBufferBindingInfo<'a> {

    fn binding_content(&self) -> &DescriptorBindingContent {
        &self.content
    }

    fn write_set(&self, set: &HaDescriptorSet) -> DescriptorWriteInfo {

        let mut buffer_infos = vec![];
        for &element_index in self.element_indices.iter() {
            let buffer_info = vk::DescriptorBufferInfo {
                // buffer is the buffer resource.
                buffer: self.buffer.handle,
                // offset is the offset in bytes from the start of buffer.
                // Access to buffer memory via this descriptor uses addressing that is relative to this starting offset.
                offset: (element_index as vkMemorySize) * self.element_size,
                // range is the size in bytes that is used for this descriptor update,
                // or vk::WHOLE_SIZE to use the range from offset to the end of the buffer.
                // TODO: check maxUniformBufferRange or maxStorageBufferRange in physical device limit.
                range : self.element_size,
            };
            buffer_infos.push(buffer_info);
        }

        let write_set = vk::WriteDescriptorSet {
            s_type: vk::StructureType::WriteDescriptorSet,
            p_next: ptr::null(),
            dst_set    : set.handle,
            dst_binding: self.content.binding,
            // TODO: Currently dst_array_element filed is not configurable
            dst_array_element  : 0,
            descriptor_count   : buffer_infos.len() as vkint,
            descriptor_type    : self.content.descriptor_type.value(),
            p_image_info       : ptr::null(),
            p_buffer_info      : buffer_infos.as_ptr(),
            p_texel_buffer_view: ptr::null(),
        };

        DescriptorWriteInfo {
            content: Box::new(DescriptorWriteBufferContent(buffer_infos)),
            info: write_set,
        }
    }
}


pub struct DescriptorImageBindingInfo<'a> {

    pub content: DescriptorBindingContent,
    /// sampler information.
    pub sampler: &'a HaSampler,
    /// what the layout is for this descriptor in shader.
    pub dst_layout: ImageLayout,
    /// the reference to image view where the descriptor data stores.
    pub item: &'a ImageViewItem,
}

struct DescriptorWriteImageContent(Vec<vk::DescriptorImageInfo>);
impl DescriptorWriteContent for DescriptorWriteImageContent {}

impl<'a> DescriptorBindingInfo for DescriptorImageBindingInfo<'a> {

    fn binding_content(&self) -> &DescriptorBindingContent {
        &self.content
    }

    fn write_set(&self, set: &HaDescriptorSet) -> DescriptorWriteInfo {

        let mut image_infos = vec![];
        for _ in 0..(self.content.count as vkMemorySize) {

            let info = vk::DescriptorImageInfo {
                sampler      : self.sampler.handle,
                image_view   : self.item.view_handle,
                image_layout : self.dst_layout.value(),
            };
            image_infos.push(info);
        }

        let write_set = vk::WriteDescriptorSet {
            s_type: vk::StructureType::WriteDescriptorSet,
            p_next: ptr::null(),
            dst_set    : set.handle,
            dst_binding: self.content.binding,
            // TODO: Currently dst_array_element filed is not configurable
            dst_array_element  : 0,
            descriptor_count   : self.content.count,
            descriptor_type    : self.content.descriptor_type.value(),
            p_image_info       : image_infos.as_ptr(),
            p_buffer_info      : ptr::null(),
            p_texel_buffer_view: ptr::null(),
        };

        DescriptorWriteInfo {
            content: Box::new(DescriptorWriteImageContent(image_infos)),
            info: write_set,
        }
    }
}

