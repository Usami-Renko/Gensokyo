
use ash::vk;
use ash::vk::uint32_t;

use pipeline::state::vertex_input::HaVertexInput;

#[derive(Debug)]
pub struct HaVertexInputBinding {
    pub binding: uint32_t,
    pub stride : uint32_t,
    pub rate   : vk::VertexInputRate,
}

#[derive(Debug)]
pub struct HaVertexInputAttribute {
    pub binding : uint32_t,
    pub location: uint32_t,
    pub format  : vk::Format,
    pub offset  : uint32_t,
}

#[derive(Debug)]
pub struct VertexInputDescription {

    pub bindings:   Vec<HaVertexInputBinding>,
    pub attributes: Vec<HaVertexInputAttribute>,
}

impl VertexInputDescription {

    pub(crate) fn desc(self) -> HaVertexInput {

        let bindings: Vec<vk::VertexInputBindingDescription> = self.bindings.iter()
            .map(|b|
                vk::VertexInputBindingDescription {
                    binding   : b.binding,
                    stride    : b.stride,
                    input_rate: b.rate,
                }
        ).collect();
        let attributes: Vec<vk::VertexInputAttributeDescription> = self.attributes.iter()
            .map(|a|
                vk::VertexInputAttributeDescription {
                    binding : a.binding,
                    location: a.location,
                    format  : a.format,
                    offset  : a.offset,
                }
        ).collect();

        HaVertexInput::setup(bindings, attributes)
    }
}
