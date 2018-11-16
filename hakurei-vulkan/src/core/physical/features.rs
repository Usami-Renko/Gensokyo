
use ash::vk;
use ash::version::InstanceV1_0;

use core::instance::HaInstance;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PhysicalFeatureType {

    RobustBufferAccess,
    FullDrawIndexUint32,
    ImageCubeArray,
    IndependentBlend,
    GeometryShader,
    TessellationShader,
    SampleRateShading,
    DualSrcBlend,
    LogicOp,
    MultiDrawIndirect,
    DrawIndirectFirstInstance,
    DepthClamp,
    DepthBiasClamp,
    /// Fill mode non solid is required for wireframe display.
    FillModeNonSolid,
    DepthBounds,
    /// Line width > 1.0f only if wide lines feature is supported and enabled.
    WideLines,
    LargePoints,
    AlphaToOne,
    MultiViewport,
    SamplerAnisotropy,
    TextureCompressionEtc2,
    TextureCompressionAstcLdr,
    TextureCompressionBc,
    OcclusionQueryPrecise,
    PipelineStatisticsQuery,
    VertexPipelineStoresAndAtomics,
    FragmentStoresAndAtomics,
    ShaderTessellationAndGeometryPointSize,
    ShaderImageGatherExtended,
    ShaderStorageImageExtendedFormats,
    ShaderStorageImageMultisample,
    ShaderStorageImageReadWithoutFormat,
    ShaderStorageImageWriteQithoutFormat,
    ShaderUniformBufferArrayDynamicIndexing,
    ShaderSampledImageArrayDynamicIndexing,
    ShaderStorageBufferArrayDynamicIndexing,
    ShaderStorageImageArrayDynamicIndexing,
    ShaderClipDistance,
    ShaderCullDistance,
    ShaderFloat64,
    ShaderInt64,
    ShaderInt16,
    ShaderResourceResidency,
    ShaderResourceMinLod,
    SparseBinding,
    SparseResidencyBuffer,
    SparseResidencyImage2d,
    SparseResidencyImage3d,
    SparseResidency2samples,
    SparseResidency4samples,
    SparseResidency8samples,
    SparseResidency16samples,
    SparseResidencyAliased,
    VariableMultisampleRate,
    InheritedQueries,
}

pub struct PhyscialFeatures {

    handle: vk::PhysicalDeviceFeatures,
    pub enables: Option<vk::PhysicalDeviceFeatures>,
}

impl PhyscialFeatures {

    pub fn inspect(instance: &HaInstance, physical_device: vk::PhysicalDevice) -> PhyscialFeatures {

        let handle = instance.handle.get_physical_device_features(physical_device);

        PhyscialFeatures {
            handle,
            enables: None,
        }
    }

    pub fn get_enable_features(&self) -> vk::PhysicalDeviceFeatures {

        if let Some(ref features) = self.enables {
            features.clone()
        } else {
            vk::PhysicalDeviceFeatures { ..Default::default() }
        }
    }
}

macro_rules! impl_physical_features {
    ($struct_name:ty, {$($feature:tt -> $vk_feature:tt,)*}) => {

        impl $struct_name {

            pub fn check_requirements(&self, require_features: &Vec<PhysicalFeatureType>) -> bool {

                require_features.iter().all(|requirement| {
                    match requirement {
                        $(| PhysicalFeatureType::$feature => self.handle.$vk_feature == 1,)*
                    }
                })
            }

            pub fn enable_features(&mut self, require_features: &Vec<PhysicalFeatureType>) {
                let mut enable_features = vk::PhysicalDeviceFeatures {
                    ..Default::default()
                };

                require_features.iter().for_each(|feature| {
                    match feature {
                        $(| PhysicalFeatureType::$feature => enable_features.$vk_feature = 1,)*
                    }
                });

                self.enables = Some(enable_features);
            }
        }
    };
}

impl_physical_features!(
    PhyscialFeatures, {
    RobustBufferAccess -> robust_buffer_access,
    FullDrawIndexUint32 -> full_draw_index_uint32,
    ImageCubeArray -> image_cube_array,
    IndependentBlend -> independent_blend,
    GeometryShader -> geometry_shader,
    TessellationShader -> tessellation_shader,
    SampleRateShading -> sample_rate_shading,
    DualSrcBlend -> dual_src_blend,
    LogicOp -> logic_op,
    MultiDrawIndirect -> multi_draw_indirect,
    DrawIndirectFirstInstance -> draw_indirect_first_instance,
    DepthClamp -> depth_clamp,
    DepthBiasClamp -> depth_bias_clamp,
    FillModeNonSolid -> fill_mode_non_solid,
    DepthBounds -> depth_bounds,
    WideLines -> wide_lines,
    LargePoints -> large_points,
    AlphaToOne -> alpha_to_one,
    MultiViewport -> multi_viewport,
    SamplerAnisotropy -> sampler_anisotropy,
    TextureCompressionEtc2 -> texture_compression_etc2,
    TextureCompressionAstcLdr -> texture_compression_astc_ldr,
    TextureCompressionBc -> texture_compression_bc,
    OcclusionQueryPrecise -> occlusion_query_precise,
    PipelineStatisticsQuery -> pipeline_statistics_query,
    VertexPipelineStoresAndAtomics -> vertex_pipeline_stores_and_atomics,
    FragmentStoresAndAtomics -> fragment_stores_and_atomics,
    ShaderTessellationAndGeometryPointSize -> shader_tessellation_and_geometry_point_size,
    ShaderImageGatherExtended -> shader_image_gather_extended,
    ShaderStorageImageExtendedFormats -> shader_storage_image_extended_formats,
    ShaderStorageImageMultisample -> shader_storage_image_multisample,
    ShaderStorageImageReadWithoutFormat -> shader_storage_image_read_without_format,
    ShaderStorageImageWriteQithoutFormat -> shader_storage_image_write_without_format,
    ShaderUniformBufferArrayDynamicIndexing -> shader_uniform_buffer_array_dynamic_indexing,
    ShaderSampledImageArrayDynamicIndexing -> shader_sampled_image_array_dynamic_indexing,
    ShaderStorageBufferArrayDynamicIndexing -> shader_storage_buffer_array_dynamic_indexing,
    ShaderStorageImageArrayDynamicIndexing -> shader_storage_image_array_dynamic_indexing,
    ShaderClipDistance -> shader_clip_distance,
    ShaderCullDistance -> shader_cull_distance,
    ShaderFloat64 -> shader_float64,
    ShaderInt64 -> shader_int64,
    ShaderInt16 -> shader_int16,
    ShaderResourceResidency -> shader_resource_residency,
    ShaderResourceMinLod -> shader_resource_min_lod,
    SparseBinding -> sparse_binding,
    SparseResidencyBuffer -> sparse_residency_buffer,
    SparseResidencyImage2d -> sparse_residency_image2d,
    SparseResidencyImage3d -> sparse_residency_image3d,
    SparseResidency2samples -> sparse_residency2samples,
    SparseResidency4samples -> sparse_residency4samples,
    SparseResidency8samples -> sparse_residency8samples,
    SparseResidency16samples -> sparse_residency16samples,
    SparseResidencyAliased -> sparse_residency_aliased,
    VariableMultisampleRate -> variable_multisample_rate,
    InheritedQueries -> inherited_queries,
});
