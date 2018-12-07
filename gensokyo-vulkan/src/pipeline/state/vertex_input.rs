
use ash::vk;

use crate::types::vkuint;
use std::ptr;

#[derive(Default)]
pub struct GsVertexInputState {

    bindings:   Vec<vk::VertexInputBindingDescription>,
    attributes: Vec<vk::VertexInputAttributeDescription>,
}

impl GsVertexInputState {

    pub fn setup(bindings: Vec<vk::VertexInputBindingDescription>, attributes: Vec<vk::VertexInputAttributeDescription>)
        -> GsVertexInputState {

        GsVertexInputState {
            bindings, attributes,
        }
    }

    pub(crate) fn info(&self) -> vk::PipelineVertexInputStateCreateInfo {

        vk::PipelineVertexInputStateCreateInfo {
            s_type : vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            p_next : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags  : vk::PipelineVertexInputStateCreateFlags::empty(),
            vertex_binding_description_count   : self.bindings.len() as vkuint,
            p_vertex_binding_descriptions      : self.bindings.as_ptr(),
            vertex_attribute_description_count : self.attributes.len() as vkuint,
            p_vertex_attribute_descriptions    : self.attributes.as_ptr(),
        }
    }

    pub fn add_binding(&mut self, binding: vk::VertexInputBindingDescription) {
        self.bindings.push(binding);
    }
    pub fn add_attribute(&mut self, attrubute: vk::VertexInputAttributeDescription) {
        self.attributes.push(attrubute);
    }
}
