
use ash::vk;

use pipeline::state::vertex_input::HaVertexInputState;

use utils::types::{ vkint, vkformat };
use utils::marker::VulkanEnum;

#[derive(Debug)]
pub struct HaVertexInputBinding {

    pub binding: vkint,
    pub stride : vkint,
    pub rate   : VertexInputRate,
}

#[derive(Debug)]
pub struct HaVertexInputAttribute {

    pub binding : vkint,
    pub location: vkint,
    pub format  : vkformat,
    pub offset  : vkint,
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
                    input_rate: b.rate.value(),
                }
        ).collect::<Vec<_>>();
        let attributes = self.attributes.iter()
            .map(|a|
                vk::VertexInputAttributeDescription {
                    binding : a.binding,
                    location: a.location,
                    format  : a.format.value(),
                    offset  : a.offset,
                }
        ).collect::<Vec<_>>();

        HaVertexInputState::setup(bindings, attributes)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VertexInputRate {

    Vertex,
    Instance,
}

impl VulkanEnum for VertexInputRate {
    type EnumType = vk::VertexInputRate;

    fn value(&self) -> Self::EnumType {
        match self {
            | VertexInputRate::Vertex   => vk::VertexInputRate::Vertex,
            | VertexInputRate::Instance => vk::VertexInputRate::Instance,
        }
    }
}
