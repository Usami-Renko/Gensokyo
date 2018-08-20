
use ash::vk;
use ash::version::InstanceV1_0;

use core::instance::Instance;

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
    FillModeNonSolid,
    DepthBounds,
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
}

impl PhyscialFeatures {

    pub fn inspect(instance: &Instance, physical_device: vk::PhysicalDevice) -> PhyscialFeatures {

        let handle = instance.handle.get_physical_device_features(physical_device);

        PhyscialFeatures {
            handle,
        }
    }

    pub fn check_requirements(&self, require_features: &Vec<PhysicalFeatureType>) -> bool {
        require_features.iter().all(|requirement| {

            match *requirement {
                | PhysicalFeatureType::RobustBufferAccess => self.handle.robust_buffer_access == 1,
                | PhysicalFeatureType::FullDrawIndexUint32 => self.handle.full_draw_index_uint32 == 1,
                | PhysicalFeatureType::ImageCubeArray => self.handle.image_cube_array == 1,
                | PhysicalFeatureType::IndependentBlend => self.handle.independent_blend == 1,
                | PhysicalFeatureType::GeometryShader => self.handle.geometry_shader == 1,
                | PhysicalFeatureType::TessellationShader => self.handle.tessellation_shader == 1,
                | PhysicalFeatureType::SampleRateShading => self.handle.sample_rate_shading == 1,
                | PhysicalFeatureType::DualSrcBlend => self.handle.dual_src_blend == 1,
                | PhysicalFeatureType::LogicOp => self.handle.logic_op == 1,
                | PhysicalFeatureType::MultiDrawIndirect => self.handle.multi_draw_indirect == 1,
                | PhysicalFeatureType::DrawIndirectFirstInstance => self.handle.draw_indirect_first_instance == 1,
                | PhysicalFeatureType::DepthClamp => self.handle.depth_clamp == 1,
                | PhysicalFeatureType::DepthBiasClamp => self.handle.depth_bias_clamp == 1,
                | PhysicalFeatureType::FillModeNonSolid => self.handle.fill_mode_non_solid == 1,
                | PhysicalFeatureType::DepthBounds => self.handle.depth_bounds == 1,
                | PhysicalFeatureType::WideLines => self.handle.wide_lines == 1,
                | PhysicalFeatureType::LargePoints => self.handle.large_points == 1,
                | PhysicalFeatureType::AlphaToOne => self.handle.alpha_to_one == 1,
                | PhysicalFeatureType::MultiViewport => self.handle.multi_viewport == 1,
                | PhysicalFeatureType::SamplerAnisotropy => self.handle.sampler_anisotropy == 1,
                | PhysicalFeatureType::TextureCompressionEtc2 => self.handle.texture_compression_etc2 == 1,
                | PhysicalFeatureType::TextureCompressionAstcLdr => self.handle.texture_compression_astc_ldr == 1,
                | PhysicalFeatureType::TextureCompressionBc => self.handle.texture_compression_bc == 1,
                | PhysicalFeatureType::OcclusionQueryPrecise => self.handle.occlusion_query_precise == 1,
                | PhysicalFeatureType::PipelineStatisticsQuery => self.handle.pipeline_statistics_query == 1,
                | PhysicalFeatureType::VertexPipelineStoresAndAtomics => self.handle.vertex_pipeline_stores_and_atomics == 1,
                | PhysicalFeatureType::FragmentStoresAndAtomics => self.handle.fragment_stores_and_atomics == 1,
                | PhysicalFeatureType::ShaderTessellationAndGeometryPointSize => self.handle.shader_tessellation_and_geometry_point_size == 1,
                | PhysicalFeatureType::ShaderImageGatherExtended => self.handle.shader_image_gather_extended == 1,
                | PhysicalFeatureType::ShaderStorageImageExtendedFormats => self.handle.shader_storage_image_extended_formats == 1,
                | PhysicalFeatureType::ShaderStorageImageMultisample => self.handle.shader_storage_image_multisample == 1,
                | PhysicalFeatureType::ShaderStorageImageReadWithoutFormat => self.handle.shader_storage_image_read_without_format == 1,
                | PhysicalFeatureType::ShaderStorageImageWriteQithoutFormat => self.handle.shader_storage_image_write_without_format == 1,
                | PhysicalFeatureType::ShaderUniformBufferArrayDynamicIndexing => self.handle.shader_uniform_buffer_array_dynamic_indexing == 1,
                | PhysicalFeatureType::ShaderSampledImageArrayDynamicIndexing => self.handle.shader_sampled_image_array_dynamic_indexing == 1,
                | PhysicalFeatureType::ShaderStorageBufferArrayDynamicIndexing => self.handle.shader_storage_buffer_array_dynamic_indexing == 1,
                | PhysicalFeatureType::ShaderStorageImageArrayDynamicIndexing => self.handle.shader_storage_image_array_dynamic_indexing == 1,
                | PhysicalFeatureType::ShaderClipDistance => self.handle.shader_clip_distance == 1,
                | PhysicalFeatureType::ShaderCullDistance => self.handle.shader_cull_distance == 1,
                | PhysicalFeatureType::ShaderFloat64 => self.handle.shader_float64 == 1,
                | PhysicalFeatureType::ShaderInt64 => self.handle.shader_int64 == 1,
                | PhysicalFeatureType::ShaderInt16 => self.handle.shader_int16 == 1,
                | PhysicalFeatureType::ShaderResourceResidency => self.handle.shader_resource_residency == 1,
                | PhysicalFeatureType::ShaderResourceMinLod => self.handle.shader_resource_min_lod == 1,
                | PhysicalFeatureType::SparseBinding => self.handle.sparse_binding == 1,
                | PhysicalFeatureType::SparseResidencyBuffer => self.handle.sparse_residency_buffer == 1,
                | PhysicalFeatureType::SparseResidencyImage2d => self.handle.sparse_residency_image2d == 1,
                | PhysicalFeatureType::SparseResidencyImage3d => self.handle.sparse_residency_image3d == 1,
                | PhysicalFeatureType::SparseResidency2samples => self.handle.sparse_residency2samples == 1,
                | PhysicalFeatureType::SparseResidency4samples => self.handle.sparse_residency4samples == 1,
                | PhysicalFeatureType::SparseResidency8samples => self.handle.sparse_residency8samples == 1,
                | PhysicalFeatureType::SparseResidency16samples => self.handle.sparse_residency16samples == 1,
                | PhysicalFeatureType::SparseResidencyAliased => self.handle.sparse_residency_aliased == 1,
                | PhysicalFeatureType::VariableMultisampleRate => self.handle.variable_multisample_rate == 1,
                | PhysicalFeatureType::InheritedQueries => self.handle.inherited_queries == 1,
            }
        })
    }
}
