
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::GsDevice;

use crate::pipeline::target::GsPipeline;
use crate::pipeline::graphics::config::GfxPipelineConfig;
use crate::pipeline::shader::{ GsShaderModule, GsShaderCI };
use crate::pipeline::shader::shaderc::{ GsShaderCompiler, ShaderCompilePrefab, ShadercConfiguration };

use crate::utils::phantom::Graphics;
use crate::error::{ VkResult, VkError };

use std::ops::{ BitAnd, BitAndAssign, BitOrAssign, BitOr };
use std::ptr;


/// Graphics Pipeline Builder.
pub struct GfxPipelineBuilder {

    device : GsDevice,
    ci_flag: PipelineCIFlags,
    shaderc: GsShaderCompiler,
}

impl GfxPipelineBuilder {

    pub fn create(device: &GsDevice) -> VkResult<GfxPipelineBuilder> {

        let builder = GfxPipelineBuilder {
            device : device.clone(),
            ci_flag: PipelineCIFlags::default(),
            shaderc: GsShaderCompiler::setup(ShaderCompilePrefab::Vulkan)?,
        };
        Ok(builder)
    }

    pub fn with_flag(&mut self, flags: PipelineCIFlags) {
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
        let pipeline_ci = pipeline_ci(&self.device, &self.ci_flag, &shader_modules, &config, &PipelineDeriveState::Independence)?;

        // build pipeline.
        let handles = unsafe {
            self.device.logic.handle.create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_ci.content], None)
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
    IndependenceLayoutDefined { layout: vk::PipelineLayout },
}

impl PipelineDeriveState {

    fn flag(&self) -> vk::PipelineCreateFlags {
        match self {
            | PipelineDeriveState::AsParent { .. } => {
                vk::PipelineCreateFlags::ALLOW_DERIVATIVES
            },
            | PipelineDeriveState::AsChildren { .. } => {
                vk::PipelineCreateFlags::DERIVATIVE
            },
            | PipelineDeriveState::IndependenceLayoutDefined { .. }
            | PipelineDeriveState::Independence => {
                vk::PipelineCreateFlags::empty()
            },
        }
    }
}
// ------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------
pub struct PipelineCIFlags(vk::PipelineCreateFlags);

impl PipelineCIFlags {
    pub const DISABLE_OPTIMIZATION: PipelineCIFlags = PipelineCIFlags(vk::PipelineCreateFlags::DISABLE_OPTIMIZATION);
    pub const DEFER_COMPILE_NV: PipelineCIFlags = PipelineCIFlags(vk::PipelineCreateFlags::DEFER_COMPILE_NV);
    pub const VIEW_INDEX_FROM_DEVICE_INDEX: PipelineCIFlags = PipelineCIFlags(vk::PipelineCreateFlags::VIEW_INDEX_FROM_DEVICE_INDEX);
    pub const DISPATCH_BASE: PipelineCIFlags = PipelineCIFlags(vk::PipelineCreateFlags::DISPATCH_BASE);

    pub(super) fn combine_derive(&self, derive: &PipelineDeriveState) -> PipelineCIFlags {
        PipelineCIFlags(self.0 | derive.flag())
    }
}

impl Default for PipelineCIFlags {

    fn default() -> PipelineCIFlags {
        PipelineCIFlags(vk::PipelineCreateFlags::empty())
    }
}

impl BitAnd for PipelineCIFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        PipelineCIFlags(self.0 & rhs.0)
    }
}

impl BitAndAssign for PipelineCIFlags {

    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for PipelineCIFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        PipelineCIFlags(self.0 | rhs.0)
    }
}

impl BitOrAssign for PipelineCIFlags {

    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
// ------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------
#[derive(Debug)]
pub(super) struct GfxPipelineCI {

    pub content: vk::GraphicsPipelineCreateInfo,
    pub shader_cis: Vec<vk::PipelineShaderStageCreateInfo>,
    pub pipeline_layout: vk::PipelineLayout,
}

pub(super) fn compile_shaders(device: &GsDevice, compiler: &mut GsShaderCompiler, shaders: &[GsShaderCI]) -> VkResult<Vec<GsShaderModule>> {

    let mut shader_modules = Vec::with_capacity(shaders.len());
    for shader in shaders.iter() {
        let module = shader.build(device, compiler)?;
        shader_modules.push(module);
    }

    Ok(shader_modules)
}

pub(super) fn destroy_modules(device: &GsDevice, modules: &[GsShaderModule]) {
    for module in modules.iter() {
        module.discard(device);
    }
}


// this function must be inline, or the ptr may be lost.
#[inline(always)]
pub(super) fn pipeline_ci(device: &GsDevice, flag: &PipelineCIFlags, shader_modules: &[GsShaderModule], config: &GfxPipelineConfig, derive_state: &PipelineDeriveState) -> VkResult<GfxPipelineCI> {

    let ci_flag = flag.combine_derive(&derive_state);

    // Don't use collect method here, or some ptr may be lost.
    let mut shader_cis = Vec::with_capacity(shader_modules.len());
    for shader_module in shader_modules.iter() {
        let shader_ci = shader_module.ci();
        shader_cis.push(shader_ci);
    }

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
        | PipelineDeriveState::IndependenceLayoutDefined { layout } => {
            (layout.clone(), vk::Pipeline::null())
        },
    };

    let pipeline_ci = vk::GraphicsPipelineCreateInfo {
        s_type: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
        p_next: ptr::null(),
        // `flags` specifies how the pipeline will be generated.
        flags : ci_flag.0,
        // `p_stages` describes the set of the shader stages to be included in the graphics pipeline.
        stage_count: shader_cis.len() as _,
        p_stages   : shader_cis.as_ptr(),
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
        shader_cis,
        pipeline_layout,
    };
    Ok(result)
}
// ------------------------------------------------------------------------------------------
