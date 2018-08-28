
use ash::vk;
use ash::vk::uint32_t;

use pipeline::state::vertex_input::HaVertexInput;
use pipeline::shader::module::HaShaderInfo;

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

pub struct VertexContent {
    pub infos      : Vec<HaShaderInfo>,
    pub description: VertexInputDescription,
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

#[macro_export]
macro_rules! define_input {
    (
    #[binding = $binding_index:expr, rate = $input_rate:ident]
    struct $struct_name:ident {
        $(
            #[location = $loc_index:expr, format = $format:ident]
            $filed_name:ident: [$field_type:ty; $element_count:expr],
        )*
    }
    ) => (

        #[derive(Debug, Clone, Copy)]
        struct $struct_name {
            $(
                $filed_name: [$field_type; $element_count],
            )*
        }

        impl $struct_name {

            fn desc() -> VertexInputDescription {
                use std::mem;
                VertexInputDescription {
                    bindings: vec![
                        HaVertexInputBinding {
                            binding: $binding_index,
                            stride: mem::size_of::<Self>() as uint32_t,
                            rate: vertex_rate!($input_rate),
                        },
                    ],
                    attributes: vec![$(
                        HaVertexInputAttribute {
                            binding: $binding_index,
                            location: $loc_index,
                            format: vk_format!($format),
                            offset: offset_of!(Self, $filed_name) as uint32_t,
                        },
                    )*],
                }
            }
        }

    )
}
