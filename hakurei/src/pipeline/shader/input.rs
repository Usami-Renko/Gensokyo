
use ash::vk;
use ash::vk::uint32_t;

use pipeline::state::HaVertexInputState;
use utility::marker::VulkanEnum;

#[derive(Debug)]
pub struct HaVertexInputBinding {

    pub binding: uint32_t,
    pub stride : uint32_t,
    pub rate   : VertexInputRate,
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

    pub(crate) fn desc(self) -> HaVertexInputState {

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
                    format  : a.format,
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
