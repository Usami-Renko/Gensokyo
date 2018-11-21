
use ash::vk;

use pipeline::state::vertex_input::HaVertexInputState;

use types::vkuint;

#[derive(Debug)]
pub struct HaVertexInputBinding {

    pub binding: vkuint,
    pub stride : vkuint,
    pub rate   : vk::VertexInputRate,
}

#[derive(Debug)]
pub struct HaVertexInputAttribute {

    pub binding : vkuint,
    pub location: vkuint,
    pub format  : vk::Format,
    pub offset  : vkuint,
}

#[derive(Debug)]
pub struct VertexInputDescription {

    pub bindings:   Vec<HaVertexInputBinding>,
    pub attributes: Vec<HaVertexInputAttribute>,
}

impl VertexInputDescription {

    pub fn desc(self) -> HaVertexInputState {

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

        HaVertexInputState::setup(bindings, attributes)
    }
}
