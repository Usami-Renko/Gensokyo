
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;
use crate::pipeline::error::PipelineError;

use crate::descriptor::GsDescriptorSetLayout;

use std::ptr;

#[derive(Default)]
pub struct PipelineLayoutBuilder {

    descriptor_layouts: Vec<vk::DescriptorSetLayout>,
    push_constants    : Vec<vk::PushConstantRange>,
}

impl PipelineLayoutBuilder {

    pub fn build(&self, device: &GsDevice) -> Result<vk::PipelineLayout, PipelineError> {

        let create_info = vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags : vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count          : self.descriptor_layouts.len() as _,
            p_set_layouts             : self.descriptor_layouts.as_ptr(),
            push_constant_range_count : self.push_constants.len() as _,
            p_push_constant_ranges    : self.push_constants.as_ptr(),
        };

        unsafe {
            device.handle.create_pipeline_layout(&create_info, None)
                .or(Err(PipelineError::LayoutCreationError))
        }
    }

    pub fn add_descriptor_layout(&mut self, layout: &GsDescriptorSetLayout) {
        self.descriptor_layouts.push(layout.handle);
    }
    pub fn add_push_constant(&mut self, constant: vk::PushConstantRange) {
        self.push_constants.push(constant);
    }
}

#[derive(Default)]
pub struct GsPipelineLayout {

    pub(crate) handle: vk::PipelineLayout,
}

impl GsPipelineLayout {

    pub fn cleanup(&self, device: &GsDevice) {
        unsafe {
            device.handle.destroy_pipeline_layout(self.handle, None);
        }
    }
}
