
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;

use crate::pipeline::target::GsPipeline;
use crate::pipeline::graphics::config::GfxPipelineConfig;
use crate::pipeline::shader::{ GsShaderModule, GsShaderInfo };
use crate::pipeline::shader::shaderc::{ GsShaderCompiler, ShaderCompilePrefab, ShadercConfiguration };

use crate::utils::phantom::Graphics;
use crate::error::{ VkResult, VkError };

use std::ops::{ BitAnd, BitAndAssign, BitOrAssign, BitOr };
use std::ptr;


/// Graphics Pipeline Builder.
pub struct GfxPipelineBuilder {

    device : GsDevice,
    ci_flag: GsPipelineCIFlags,
    shaderc: GsShaderCompiler,
}

impl GfxPipelineBuilder {

    pub fn new(device: &GsDevice) -> VkResult<GfxPipelineBuilder> {

        let builder = GfxPipelineBuilder {
            device : device.clone(),
            ci_flag: GsPipelineCIFlags::default(),
            shaderc: GsShaderCompiler::setup(ShaderCompilePrefab::Vulkan)?,
        };
        Ok(builder)
    }

    pub fn with_flag(&mut self, flags: GsPipelineCIFlags) {
        self.ci_flag |= flags;
    }

    pub fn set_shaderc(&mut self, configuration: ShadercConfiguration) -> VkResult<()> {

        self.shaderc = GsShaderCompiler::from_configuration(configuration)?;

        Ok(())
    }

    pub fn build(&mut self, config: GfxPipelineConfig) -> VkResult<GsPipeline<Graphics>> {

        // compile shader.
        let shader_modules = compile_shaders(&self.device, &mut self.shaderc, &config.shaders)?;
        // generate create info.
        let derive_state = PipelineDeriveState::Independence;
        let pipeline_ci = pipeline_ci(&self.device, &self.ci_flag, &shader_modules, &config, &derive_state)?;

        // build pipeline.
        let handles = unsafe {
            self.device.handle.create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_ci.content], None)
                .or(Err(VkError::create("Graphics Pipelines")))?
        };
        destroy_modules(&self.device, &shader_modules);

        let result = GsPipeline::new(self.device.clone(), handles[0], pipeline_ci.pipeline_layout, config.render_pass);
        Ok(result)
    }
}


// ------------------------------------------------------------------------------------------
pub(super) enum PipelineDeriveState {
    AsParent { layout: vk::PipelineLayout },
    AsChildren { parent: vk::Pipeline, layout: vk::PipelineLayout },
    Independence,
}

impl PipelineDeriveState {

    fn flag(&self) -> vk::PipelineCreateFlags {
        match self {
            | PipelineDeriveState::AsParent { .. }   => vk::PipelineCreateFlags::ALLOW_DERIVATIVES,
            | PipelineDeriveState::AsChildren { .. } => vk::PipelineCreateFlags::DERIVATIVE,
            | PipelineDeriveState::Independence      => vk::PipelineCreateFlags::empty(),
        }
    }
}
// ------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------
pub struct GsPipelineCIFlags(vk::PipelineCreateFlags);

impl GsPipelineCIFlags {
    pub const DISABLE_OPTIMIZATION: GsPipelineCIFlags = GsPipelineCIFlags(vk::PipelineCreateFlags::DISABLE_OPTIMIZATION);
    pub const DEFER_COMPILE_NV: GsPipelineCIFlags = GsPipelineCIFlags(vk::PipelineCreateFlags::DEFER_COMPILE_NV);
    pub const VIEW_INDEX_FROM_DEVICE_INDEX: GsPipelineCIFlags = GsPipelineCIFlags(vk::PipelineCreateFlags::VIEW_INDEX_FROM_DEVICE_INDEX);
    pub const DISPATCH_BASE: GsPipelineCIFlags = GsPipelineCIFlags(vk::PipelineCreateFlags::DISPATCH_BASE);

    pub(super) fn combine_derive(&self, derive: &PipelineDeriveState) -> GsPipelineCIFlags {
        GsPipelineCIFlags(self.0 | derive.flag())
    }
}

impl Default for GsPipelineCIFlags {

    fn default() -> GsPipelineCIFlags {
        GsPipelineCIFlags(vk::PipelineCreateFlags::empty())
    }
}

impl BitAnd for GsPipelineCIFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        GsPipelineCIFlags(self.0 & rhs.0)
    }
}

impl BitAndAssign for GsPipelineCIFlags {

    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for GsPipelineCIFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        GsPipelineCIFlags(self.0 | rhs.0)
    }
}

impl BitOrAssign for GsPipelineCIFlags {

    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
// ------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------
#[derive(Debug)]
pub(super) struct GfxPipelineCI {

    pub content: vk::GraphicsPipelineCreateInfo,
    pub shader_ci: Vec<vk::PipelineShaderStageCreateInfo>,
    pub pipeline_layout: vk::PipelineLayout,
}

pub(super) fn compile_shaders(device: &GsDevice, compiler: &mut GsShaderCompiler, shaders: &[GsShaderInfo]) -> VkResult<Vec<GsShaderModule>> {

    let mut shader_modules = Vec::with_capacity(shaders.len());
    for shader in shaders.iter() {
        let module = shader.build(device, compiler)?;
        shader_modules.push(module);
    }

    Ok(shader_modules)
}

pub(super) fn destroy_modules(device: &GsDevice, modules: &[GsShaderModule]) {
    for module in modules.iter() {
        module.destroy(device);
    }
}

#[inline(always)]
pub(super) fn pipeline_ci(device: &GsDevice, flag: &GsPipelineCIFlags, shader_modules: &[GsShaderModule], config: &GfxPipelineConfig, derive_state: &PipelineDeriveState) -> VkResult<GfxPipelineCI> {

    let ci_flag = flag.combine_derive(&derive_state);

    let shader_ci: Vec<vk::PipelineShaderStageCreateInfo> = shader_modules.iter()
        .map(|m| m.ci()).collect();
    let tessellation_ci = config.states.tessellation.as_ref()
        .map_or(ptr::null(), |t| &t.ci());
    let dynamic_ci = if config.states.dynamic.is_contain_state() {
        &config.states.dynamic.ci() } else { ptr::null() };

    let (pipeline_layout, base_pipeline) = match derive_state {
        | PipelineDeriveState::AsChildren { parent, layout } => {
            (layout.clone(), parent.clone())
        },
        | PipelineDeriveState::AsParent { layout } => {
            (layout.clone(), vk::Pipeline::null())
        },
        | PipelineDeriveState::Independence => {
            let layout = config.layout_builder.build(device)?;
            (layout, vk::Pipeline::null())
        },
    };

    let pipeline_ci = vk::GraphicsPipelineCreateInfo {
        s_type: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
        p_next: ptr::null(),
        // `flags` specifies how the pipeline will be generated.
        flags : ci_flag.0,
        // `p_stages` describes the set of the shader stages to be included in the graphics pipeline.
        stage_count: shader_ci.len() as _,
        p_stages   : shader_ci.as_ptr(),
        p_vertex_input_state  : &config.states.vertex_input.ci(),
        p_input_assembly_state: &config.states.input_assembly.ci(),
        p_viewport_state      : &config.states.viewport.ci(),
        p_rasterization_state : &config.states.rasterizer.ci(),
        p_multisample_state   : &config.states.multisample.ci(),
        p_depth_stencil_state : &config.states.depth_stencil.ci(),
        p_color_blend_state   : &config.states.blend.ci(),
        p_tessellation_state  : tessellation_ci,
        p_dynamic_state       : dynamic_ci,
        // `layout` is the description of binding locations used by both the pipeline and descriptor sets used with the pipeline.
        layout     : pipeline_layout,
        render_pass: config.render_pass.handle,
        // TODO: Add configuration for this field.
        // `subpass` the index of the subpass in the render pass where this pipeline will be used.
        subpass: 0,
        /// `base_pipeline_handle` is the pipeline to derive from.
        base_pipeline_handle: base_pipeline,
        base_pipeline_index: -1,
    };

    let result = GfxPipelineCI {
        content: pipeline_ci,
        shader_ci,
        pipeline_layout,
    };
    Ok(result)
}
// ------------------------------------------------------------------------------------------
