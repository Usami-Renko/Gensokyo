
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;
use pipeline::error::PipelineError;

use resources::descriptor::HaDescriptorSetLayout;
use utils::types::vkint;

use std::ptr;

pub struct PipelineLayoutBuilder {

    descriptor_layouts: Vec<vk::DescriptorSetLayout>,
    push_constants    : Vec<vk::PushConstantRange>,
}

impl PipelineLayoutBuilder {

    pub fn build(&self, device: &HaDevice) -> Result<vk::PipelineLayout, PipelineError> {
        let create_info = self.info();

        unsafe {
            device.handle.create_pipeline_layout(&create_info, None)
                .or(Err(PipelineError::LayoutCreationError))
        }
    }

    fn info(&self) -> vk::PipelineLayoutCreateInfo {
        vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PipelineLayoutCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags : vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: self.descriptor_layouts.len() as vkint,
            p_set_layouts   : self.descriptor_layouts.as_ptr(),
            push_constant_range_count: self.push_constants.len() as vkint,
            p_push_constant_ranges   : self.push_constants.as_ptr(),
        }
    }

    pub fn add_descriptor_layout(&mut self, layout: &HaDescriptorSetLayout) {
        self.descriptor_layouts.push(layout.handle);
    }
    pub fn add_push_constant(&mut self, constant: vk::PushConstantRange) {
        self.push_constants.push(constant);
    }
}

impl Default for PipelineLayoutBuilder {

    fn default() -> PipelineLayoutBuilder {
        PipelineLayoutBuilder {
            descriptor_layouts: vec![],
            push_constants    : vec![],
        }
    }
}

pub struct HaPipelineLayout {

    pub(crate) handle: vk::PipelineLayout,
}

impl HaPipelineLayout {

    pub fn uninitialize() -> HaPipelineLayout {
        HaPipelineLayout {
            handle: vk::PipelineLayout::null(),
        }
    }

    pub(super) fn new(handle: vk::PipelineLayout) -> HaPipelineLayout {
        HaPipelineLayout { handle }
    }

    pub(super) fn cleanup(&self, device: &HaDevice) {
        unsafe {
            device.handle.destroy_pipeline_layout(self.handle, None);
        }
    }
}
