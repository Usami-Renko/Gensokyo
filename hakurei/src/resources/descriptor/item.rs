
use ash::vk;
use ash::vk::uint32_t;

use resources::descriptor::layout::{ BufferDescriptorType, ImageDescriptorType, DescriptorSetLayoutFlag };
use resources::descriptor::HaDescriptorSet;
use resources::buffer::BufferSubItem;
use resources::image::{ ImageViewItem, ImageLayout };
use resources::error::{ DescriptorError, DescriptorResourceError };

use pipeline::shader::ShaderStageFlag;

use utility::marker::{ VulkanFlags, VulkanEnum };

use std::ptr;

pub(crate) trait DescriptorBindingInfo {

    // TODO: remove the following three functions
    fn binding_value(&self)    -> uint32_t;
    fn descriptor_type(&self)  -> vk::DescriptorType;
    fn descritpor_count(&self) -> uint32_t;

    fn write_set(&self, set: &HaDescriptorSet) -> Result<vk::WriteDescriptorSet, DescriptorError>;
}

// TODO: Make this struct to pub(crate)
pub struct DescriptorBufferBindingInfo {

    /// the binding index used in shader for the descriptor.
    pub binding: uint32_t,
    /// the element count of each descriptor.
    pub count  : uint32_t,
    // TODO: Limit to specific buffer type
    /// the type of descriptor.
    pub type_  : BufferDescriptorType,
    /// the size of each element of descriptor.
    pub element_size: vk::DeviceSize,
    /// the reference to buffer where the descriptor data stores.
    pub buffer : BufferSubItem,
}

impl DescriptorBindingInfo for DescriptorBufferBindingInfo {

    fn binding_value(&self)    -> uint32_t { self.binding }
    fn descriptor_type(&self)  -> vk::DescriptorType { self.type_.value() }
    fn descritpor_count(&self) -> uint32_t { self.count }

    fn write_set(&self, set: &HaDescriptorSet) -> Result<vk::WriteDescriptorSet, DescriptorError> {

        let mut buffer_infos = vec![];
        for i in 0..(self.count as vk::DeviceSize) {
            let buffer_info = vk::DescriptorBufferInfo {
                // buffer is the buffer resource.
                buffer: self.buffer.handle,
                // offset is the offset in bytes from the start of buffer.
                // Access to buffer memory via this descriptor uses addressing that is relative to this starting offset.
                offset: self.buffer.offset + i * self.element_size,
                // range is the size in bytes that is used for this descriptor update, or VK_WHOLE_SIZE to use the range from offset to the end of the buffer.
                range : self.element_size,
            };
            buffer_infos.push(buffer_info);
        }

        let write_set = vk::WriteDescriptorSet {
            s_type: vk::StructureType::WriteDescriptorSet,
            p_next: ptr::null(),
            dst_set    : set.handle,
            dst_binding: self.binding,
            // TODO: Currently dst_array_element filed is not configurable
            dst_array_element   : 0,
            descriptor_count    : self.count,
            descriptor_type     : self.type_.value(),
            p_image_info        : ptr::null(),
            p_buffer_info       : buffer_infos.as_ptr(),
            p_texel_buffer_view : ptr::null(),
        };

        Ok(write_set)
    }
}


// TODO: Make this struct to pub(crate)
pub struct DescriptorImageBindingInfo {

    /// the binding index used in shader for the descriptor.
    pub(crate) binding: uint32_t,
    /// the element count of each descriptor.
    pub(crate) count  : uint32_t,
    // TODO: Limit to specific buffer type
    /// the type of descritpor.
    pub(crate) type_  : ImageDescriptorType,
    /// sampler information.
    pub(crate) sampler: vk::Sampler,
    /// what the layout is for this descriptor in shader.
    pub(crate) dst_layout: ImageLayout,
    /// the reference to image view where the descriptor data stores.
    pub(crate) view_item: ImageViewItem,
}

impl DescriptorBindingInfo for DescriptorImageBindingInfo {

    fn binding_value(&self)    -> uint32_t { self.binding }
    fn descriptor_type(&self)  -> vk::DescriptorType { self.type_.value() }
    fn descritpor_count(&self) -> uint32_t { self.count }

    fn write_set(&self, set: &HaDescriptorSet) -> Result<vk::WriteDescriptorSet, DescriptorError> {

        let mut image_infos = vec![];
        for _ in 0..(self.count as vk::DeviceSize) {

            let view_handle = self.view_item.get_view_handle()
                .ok_or(DescriptorError::Resource(DescriptorResourceError::ImageNotAllocated))?;

            let info = vk::DescriptorImageInfo {
                sampler      : self.sampler,
                image_view   : view_handle,
                image_layout : self.dst_layout.value(),
            };
            image_infos.push(info);
        }

        let write_set = vk::WriteDescriptorSet {
            s_type: vk::StructureType::WriteDescriptorSet,
            p_next: ptr::null(),
            dst_set    : set.handle,
            dst_binding: self.binding,
            // TODO: Currently dst_array_element filed is not configurable
            dst_array_element  : 0,
            descriptor_count   : self.count,
            descriptor_type    : self.type_.value(),
            p_image_info       : image_infos.as_ptr(),
            p_buffer_info      : ptr::null(),
            p_texel_buffer_view: ptr::null(),
        };

        Ok(write_set)
    }
}


pub struct DescriptorSetConfig {

    pub(crate) bindings    : Vec<Box<DescriptorBindingInfo>>,
    pub(crate) stage_flags : Vec<vk::ShaderStageFlags>,
    pub(crate) layout_flags: vk::DescriptorSetLayoutCreateFlags,
}

impl DescriptorSetConfig {

    pub fn init(flags: &[DescriptorSetLayoutFlag]) -> DescriptorSetConfig {
        DescriptorSetConfig {
            bindings    : vec![],
            stage_flags : vec![],
            layout_flags: flags.flags(),
        }
    }

    pub fn add_buffer_binding(&mut self, binding_info: DescriptorBufferBindingInfo, stages: &[ShaderStageFlag]) -> usize {
        self.add_binding(Box::new(binding_info), stages)
    }

    pub fn add_image_binding(&mut self, binding_info: DescriptorImageBindingInfo, stages: &[ShaderStageFlag]) -> usize {
        self.add_binding(Box::new(binding_info), stages)
    }

    fn add_binding(&mut self, binding: Box<DescriptorBindingInfo>, stages: &[ShaderStageFlag]) -> usize {
        let descriptor_index = self.bindings.len();
        self.bindings.push(binding);
        self.stage_flags.push(stages.flags());
        descriptor_index
    }
}


#[derive(Debug, Clone)]
pub struct DescriptorItem {

    pub(crate) set_index    : usize,
    pub(crate) binding_index: usize,
}

impl DescriptorItem {

    pub fn unset() -> DescriptorItem {
        DescriptorItem {
            set_index    : 0,
            binding_index: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DescriptorSetItem {

    pub(crate) set_index: usize,
}

impl DescriptorSetItem {

    pub fn unset() -> DescriptorSetItem {
        DescriptorSetItem {
            set_index: 0,
        }
    }
}
