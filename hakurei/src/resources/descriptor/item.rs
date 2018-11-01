
use ash::vk;
use ash::vk::uint32_t;

use resources::descriptor::layout::{ BufferDescriptorType, ImageDescriptorType, DescriptorSetLayoutFlag };
use resources::descriptor::HaDescriptorSet;
use resources::buffer::BufferItem;
use resources::image::{ ImageViewItem, ImageLayout };

use pipeline::shader::ShaderStageFlag;

use utility::wrapper::VKWrapperInfo;
use utility::marker::{ VulkanFlags, VulkanEnum };

use std::ptr;

pub(crate) type DescriptorWriteInfo = VKWrapperInfo<DescriptorWriteContent, vk::WriteDescriptorSet>;

pub(crate) trait DescriptorBindingInfo {

    // TODO: remove the following three functions
    fn binding_value(&self)    -> uint32_t;
    fn descriptor_type(&self)  -> vk::DescriptorType;
    fn descritpor_count(&self) -> uint32_t;

    fn write_set(&self, set: &HaDescriptorSet) -> DescriptorWriteInfo;
}

pub(crate) trait DescriptorWriteContent {}

pub trait DescriptorBufferBindableTarget {
    fn binding_info(&self, sub_block_indices: Option<Vec<uint32_t>>) -> DescriptorBufferBindingInfo;
}
pub trait DescriptorImageBindableTarget {
    fn binding_info(&self) -> DescriptorImageBindingInfo;
}

pub struct DescriptorBufferBindingInfo {

    /// the type of descriptor.
    pub typ: BufferDescriptorType,
    /// the binding index used in shader for the descriptor.
    pub binding: uint32_t,
    /// the total element count of each descriptor.
    pub count: uint32_t,
    /// the element index of each descriptor to update.
    pub element_indices: Vec<uint32_t>,
    /// the size of each element of descriptor.
    pub element_size: vk::DeviceSize,
    /// the reference to buffer where the descriptor data stores.
    pub buffer: BufferItem,
}

struct DescriptorWriteBufferContent(Vec<vk::DescriptorBufferInfo>);
impl DescriptorWriteContent for DescriptorWriteBufferContent {}

impl DescriptorBindingInfo for DescriptorBufferBindingInfo {

    fn binding_value(&self)    -> uint32_t { self.binding }
    fn descriptor_type(&self)  -> vk::DescriptorType { self.typ.value() }
    fn descritpor_count(&self) -> uint32_t { self.count }

    fn write_set(&self, set: &HaDescriptorSet) -> DescriptorWriteInfo {

        let mut buffer_infos = vec![];
        for &element_index in self.element_indices.iter() {
            let buffer_info = vk::DescriptorBufferInfo {
                // buffer is the buffer resource.
                buffer: self.buffer.handle,
                // offset is the offset in bytes from the start of buffer.
                // Access to buffer memory via this descriptor uses addressing that is relative to this starting offset.
                offset: (element_index as vk::DeviceSize) * self.element_size,
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
            dst_binding: self.binding,
            // TODO: Currently dst_array_element filed is not configurable
            dst_array_element  : 0,
            descriptor_count   : buffer_infos.len() as uint32_t,
            descriptor_type    : self.typ.value(),
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

    /// the type of descritpor.
    pub(crate) type_: ImageDescriptorType,
    /// the binding index used in shader for the descriptor.
    pub(crate) binding: uint32_t,
    /// the element count of each descriptor.
    pub(crate) count: uint32_t,
    /// sampler information.
    pub(crate) sampler: vk::Sampler,
    /// what the layout is for this descriptor in shader.
    pub(crate) dst_layout: ImageLayout,
    /// the reference to image view where the descriptor data stores.
    pub(crate) item: ImageViewItem,
}

struct DescriptorWriteImageContent(Vec<vk::DescriptorImageInfo>);
impl DescriptorWriteContent for DescriptorWriteImageContent {}

impl DescriptorBindingInfo for DescriptorImageBindingInfo {

    fn binding_value(&self)    -> uint32_t { self.binding }
    fn descriptor_type(&self)  -> vk::DescriptorType { self.type_.value() }
    fn descritpor_count(&self) -> uint32_t { self.count }

    fn write_set(&self, set: &HaDescriptorSet) -> DescriptorWriteInfo {

        let mut image_infos = vec![];
        for _ in 0..(self.count as vk::DeviceSize) {

            let info = vk::DescriptorImageInfo {
                sampler      : self.sampler,
                image_view   : self.item.view_handle,
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

        DescriptorWriteInfo {
            content: Box::new(DescriptorWriteImageContent(image_infos)),
            info: write_set,
        }
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

    pub fn add_buffer_binding(&mut self, bind_target: &impl DescriptorBufferBindableTarget, stages: &[ShaderStageFlag]) -> usize {

        let binding_info = bind_target.binding_info(None);
        let descriptor_index = self.add_binding(
            Box::new(binding_info),
            stages
        );

        descriptor_index
    }

    pub fn add_image_binding(&mut self, bind_target: &impl DescriptorImageBindableTarget, stages: &[ShaderStageFlag]) -> usize {

        let binding_info = bind_target.binding_info();
        let descriptor_index = self.add_binding(
            Box::new(binding_info),
            stages
        );

        descriptor_index
    }

    fn add_binding(&mut self, binding: Box<DescriptorBindingInfo>, stages: &[ShaderStageFlag]) -> usize {

        let descriptor_index = self.bindings.len();
        self.bindings.push(binding);
        self.stage_flags.push(stages.flags());

        descriptor_index
    }
}


#[derive(Debug, Clone, Default)]
pub struct DescriptorItem {

    pub(crate) set_index    : usize,
    pub(crate) binding_index: usize,
}

impl DescriptorItem {

    pub fn unset() -> DescriptorItem {
        DescriptorItem::default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct DescriptorSetItem {

    pub(crate) set_index: usize,
}

impl DescriptorSetItem {

    pub fn unset() -> DescriptorSetItem {
        DescriptorSetItem::default()
    }
}
