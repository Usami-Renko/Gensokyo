
use ash::vk;
use ash::vk::uint32_t;

use pipeline::state::HaVertexInput;

pub struct HaVertexInputBinding {
    pub binding: uint32_t,
    pub stride : uint32_t,
    pub rate   : vk::VertexInputRate
}

pub struct HaVertexInputAttribute {
    pub binding : uint32_t,
    pub location: uint32_t,
    pub format  : vk::Format,
    pub offset  : uint32_t,
}

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

#[macro_export]
macro_rules! define_input {
    (
    #[binding = $binding_index:expr, rate = $input_rate:path]
    struct $struct_name:ident {
        $(
            #[location = $loc_index:expr, format = $format:path]
            $filed_name:ident: [$field_type:ty; $element_count:expr],
        )*
    }
    ) => (

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
                            // TODO: Remove vertex_rate! from prelude
                            rate: vertex_rate!($input_rate),
                        },
                    ],
                    attributes: vec![$(
                        HaVertexInputAttribute {
                            binding: $binding_index,
                            location: $loc_index,
                            // TODO: Remove vk_format! from prelude
                            format: vk_format!($format),
                            offset: offset_of!(Self, $filed_name) as uint32_t,
                        },
                    )*],
                }
            }
        }

    )
}
