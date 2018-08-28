
#[macro_export]
macro_rules! vk_format {
    (float)  => (Format::R32Sfloat);
    (double) => (Format::R64Sfloat);
    (vec2)   => (Format::R32g32Sfloat);
    (vec3)   => (Format::R32g32b32Sfloat);
    (vec4)   => (Format::R32g32b32a32Sfloat);
    (ivec2)  => (Format::R32g32Sint);
    (uvec4)  => (Format::R32g32b32a32Sint);
}

#[macro_export]
macro_rules! vertex_rate {
    (vertex)   => (VertexInputRate::Vertex);
    (instance) => (VertexInputRate::Instance);
}

// the macro is copy from crate 'memoffset' v0.2
#[macro_export]
macro_rules! offset_of {
    ($father:ty, $($field:tt)+) => ({
        #[allow(unused_unsafe)]
        let root: $father = unsafe { ::std::mem::uninitialized() };

        let base = &root as *const _ as usize;

        // Future error: borrow of packed field requires unsafe function or block (error E0133)
        #[allow(unused_unsafe)]
        let member =  unsafe { &root.$($field)* as *const _ as usize };

        ::std::mem::forget(root);

        member - base
    });
}