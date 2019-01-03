
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
        pub struct $struct_name {
            $(
                $filed_name: [$field_type; $element_count],
            )*
        }

        impl $struct_name {

            pub fn desc() -> VertexInputDescription {
                use std::mem;
                VertexInputDescription {
                    bindings: vec![
                        GsVertexInputBinding {
                            binding: $binding_index,
                            stride: mem::size_of::<Self>() as _,
                            rate: vertex_rate!($input_rate),
                        },
                    ],
                    attributes: vec![$(
                        GsVertexInputAttribute {
                            binding: $binding_index,
                            location: $loc_index,
                            format: vk_format!($format),
                            offset: offset_of!(Self, $filed_name) as _,
                        },
                    )*],
                }
            }
        }

    )
}

#[macro_export]
macro_rules! vk_format {
    (float)  => (vk::Format::D32_SFLOAT);
    (double) => (vk::Format::R64_SFLOAT);
    (vec2)   => (vk::Format::R32G32_SFLOAT);
    (vec3)   => (vk::Format::R32G32B32_SFLOAT);
    (vec4)   => (vk::Format::R32G32B32A32_SFLOAT);
    (ivec2)  => (vk::Format::R32G32_SINT);
    (uvec4)  => (vk::Format::R32G32B32A32_SINT);
}

#[macro_export]
macro_rules! vertex_rate {
    (vertex)   => (vk::VertexInputRate::VERTEX);
    (instance) => (vk::VertexInputRate::INSTANCE);
}
