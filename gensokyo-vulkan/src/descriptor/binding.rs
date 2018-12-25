
use ash::vk;

use crate::descriptor::set::GsDescriptorSet;
use crate::descriptor::types::GsDescriptorType;

use crate::types::{ vkuint, vkbytes };
use crate::utils::wrapper::VKWrapperInfo;

use std::ptr;

pub type DescriptorWriteInfo = VKWrapperInfo<DescriptorWriteContent, vk::WriteDescriptorSet>;

#[derive(Debug, Clone)]
pub struct DescriptorBindingContent {

    /// the binding index used in shader for the descriptor.
    pub binding: vkuint,
    /// the total element count of each descriptor.
    pub count: vkuint,
    /// the type of descriptor.
    pub descriptor_type: GsDescriptorType,
}

pub trait DescriptorBindingInfo {

    fn borrow_binding_content(&self) -> &DescriptorBindingContent;
    fn write_set(&self, set: &GsDescriptorSet) -> DescriptorWriteInfo;
}

pub trait DescriptorWriteContent {}

pub trait DescriptorBufferBindableTarget {
    fn binding_info(&self, sub_block_indices: Option<Vec<vkuint>>) -> DescriptorBufferBindingInfo;
}
pub trait DescriptorImageBindableTarget {
    fn binding_info(&self) -> DescriptorImageBindingInfo;
}

pub struct DescriptorBufferBindingInfo {

    pub content: DescriptorBindingContent,
    /// the element index of each descriptor to update.
    pub element_indices: Vec<vkuint>,
    /// the size of each element of descriptor.
    pub element_size: vkbytes,
    /// the handle of buffer where the descriptor data stores.
    pub buffer_handle: vk::Buffer,
}

struct DescriptorWriteBufferContent(Vec<vk::DescriptorBufferInfo>);
impl DescriptorWriteContent for DescriptorWriteBufferContent {}

impl DescriptorBindingInfo for DescriptorBufferBindingInfo {

    fn borrow_binding_content(&self) -> &DescriptorBindingContent {
        &self.content
    }

    fn write_set(&self, set: &GsDescriptorSet) -> DescriptorWriteInfo {

        let mut buffer_infos = vec![];
        for &element_index in self.element_indices.iter() {
            let buffer_info = vk::DescriptorBufferInfo {
                // buffer is the buffer resource.
                buffer: self.buffer_handle,
                // offset is the offset in bytes from the start of buffer.
                // Access to buffer memory via this descriptor uses addressing that is relative to this starting offset.
                offset: (element_index as vkbytes) * self.element_size,
                // range is the size in bytes that is used for this descriptor update,
                // or vk::WHOLE_SIZE to use the range from offset to the end of the buffer.
                // TODO: check maxUniformBufferRange or maxStorageBufferRange in physical device limit.
                range : self.element_size,
            };
            buffer_infos.push(buffer_info);
        }

        let write_set = vk::WriteDescriptorSet {
            s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
            p_next: ptr::null(),
            dst_set    : set.handle,
            dst_binding: self.content.binding,
            // TODO: Currently dst_array_element filed is not configurable
            dst_array_element  : 0,
            descriptor_count   : buffer_infos.len() as _,
            descriptor_type    : self.content.descriptor_type.to_raw(),
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


pub struct DescriptorImageBindingInfo {

    pub content: DescriptorBindingContent,
    /// the handle of sampler.
    pub sampler_handle: vk::Sampler,
    /// what the layout is for this descriptor in shader.
    pub dst_layout: vk::ImageLayout,
    /// the handle of image view where the descriptor data stores.
    pub view_handle: vk::ImageView,
}

struct DescriptorWriteImageContent(Vec<vk::DescriptorImageInfo>);
impl DescriptorWriteContent for DescriptorWriteImageContent {}

impl DescriptorBindingInfo for DescriptorImageBindingInfo {

    fn borrow_binding_content(&self) -> &DescriptorBindingContent {
        &self.content
    }

    fn write_set(&self, set: &GsDescriptorSet) -> DescriptorWriteInfo {

        let mut image_infos = vec![];
        for _ in 0..(self.content.count as vkbytes) {

            let info = vk::DescriptorImageInfo {
                sampler      : self.sampler_handle,
                image_view   : self.view_handle,
                image_layout : self.dst_layout,
            };
            image_infos.push(info);
        }

        let write_set = vk::WriteDescriptorSet {
            s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
            p_next: ptr::null(),
            dst_set    : set.handle,
            dst_binding: self.content.binding,
            // TODO: Currently dst_array_element filed is not configurable
            dst_array_element  : 0,
            descriptor_count   : self.content.count,
            descriptor_type    : self.content.descriptor_type.to_raw(),
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

