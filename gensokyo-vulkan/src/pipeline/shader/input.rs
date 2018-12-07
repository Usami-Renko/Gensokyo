
use ash::vk;

use crate::pipeline::state::vertex_input::GsVertexInputState;

use crate::types::vkuint;

#[derive(Debug)]
pub struct GsVertexInputBinding {

    pub binding: vkuint,
    pub stride : vkuint,
    pub rate   : vk::VertexInputRate,
}

#[derive(Debug)]
pub struct GsVertexInputAttribute {

    pub binding : vkuint,
    pub location: vkuint,
    pub format  : vk::Format,
    pub offset  : vkuint,
}

#[derive(Debug)]
pub struct VertexInputDescription {

    pub bindings:   Vec<GsVertexInputBinding>,
    pub attributes: Vec<GsVertexInputAttribute>,
}

impl VertexInputDescription {

    pub fn desc(self) -> GsVertexInputState {

        let bindings = self.bindings.iter()
            .map(|b|
                vk::VertexInputBindingDescription {
                    binding   : b.binding,
                    stride    : b.stride,
                    input_rate: b.rate,
                }
        ).collect();

        let attributes = self.attributes.iter()
            .map(|a|
                vk::VertexInputAttributeDescription {
                    binding : a.binding,
                    location: a.location,
                    format  : a.format,
                    offset  : a.offset,
                }
        ).collect();

        GsVertexInputState::setup(bindings, attributes)
    }
}
