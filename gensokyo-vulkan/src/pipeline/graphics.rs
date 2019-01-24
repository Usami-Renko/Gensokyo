
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;

use crate::pipeline::{
    shader::{ GsShaderModule, GsShaderInfo, VertexInputDescription },
    shader::shaderc::{ GsShaderCompiler, ShaderCompilePrefab, ShadercConfiguration },
    state::PipelineStates,
    state::vertex_input::GsVertexInputState,
    state::input_assembly::GsInputAssemblyState,
    state::viewport::ViewportStateType,
    state::rasterizer::GsRasterizerState,
    state::multisample::GsMultisampleState,
    state::depth_stencil::GsDepthStencilState,
    state::blend::GsBlendState,
    state::tessellation::GsTessellationState,
    pass::GsRenderPass,
    target::GsPipeline,
    layout::{ PipelineLayoutBuilder, GsPushConstantRange },
};

use crate::descriptor::DescriptorSet;
use crate::error::{ VkResult, VkError };
use crate::types::vkDim2D;
use crate::utils::phantom::Graphics;

use std::ops::{ BitAnd, BitAndAssign, BitOrAssign, BitOr };
use std::ptr;

// ------------------------------------------------------------------------------------------
pub struct GraphicsPipelineBuilder<'a> {

    device : GsDevice,
    ci_flag: GsPipelineCIFlags,
    configs: Vec<PipelineConfigTmp<'a>>,
    shaderc: GsShaderCompiler,
}

struct PipelineConfigTmp<'a> {
    content: &'a GraphicsPipelineConfig,
    modules: Vec<GsShaderModule>,
}

impl<'a, 'c: 'a> GraphicsPipelineBuilder<'a> {

    pub fn new(device: &GsDevice) -> VkResult<GraphicsPipelineBuilder<'a>> {

        let builder = GraphicsPipelineBuilder {
            device : device.clone(),
            ci_flag: GsPipelineCIFlags::empty(),
            configs: vec![],
            shaderc: GsShaderCompiler::setup(ShaderCompilePrefab::Vulkan)?,
        };

        Ok(builder)
    }

    pub fn with_flag(&mut self, flags: GsPipelineCIFlags) {
        self.ci_flag |= flags;
    }

    pub fn set_shaderc(&mut self, configuration: ShadercConfiguration) -> VkResult<()> {

        self.shaderc = GsShaderCompiler::setup_from_configuration(configuration)?;
        Ok(())
    }

    pub fn add_config(&mut self, pipeline_config: &'c GraphicsPipelineConfig) -> VkResult<()> {

        let mut shader_modules = Vec::with_capacity(pipeline_config.shaders.len());
        for shader in pipeline_config.shaders.iter() {
            let module = shader.build(&self.device, &mut self.shaderc)?;
            shader_modules.push(module);
        }

        self.configs.push(PipelineConfigTmp {
            content: pipeline_config,
            modules: shader_modules,
        });
        Ok(())
    }

    pub fn build(&self, derive: PipelineDeriveState) -> VkResult<Vec<GsPipeline<Graphics>>> {

        let pipeline_count = self.configs.len();
        let ci_flag = self.ci_flag.combine_derive(&derive);

        let mut layouts = Vec::with_capacity(pipeline_count);
        let mut _shader_infos = Vec::with_capacity(pipeline_count);
        let mut infos = Vec::with_capacity(pipeline_count);

        for config in self.configs.iter() {

            let shader_create_infos: Vec<vk::PipelineShaderStageCreateInfo> = config.modules.iter()
                .map(|m| m.info()).collect();
            let tessellation_info = config.content.states.tessellation.as_ref()
                .map_or(ptr::null(), |t| &t.info());
            let dynamic_info = if config.content.states.dynamic.is_contain_state() {
                &config.content.states.dynamic.info() } else { ptr::null() };

            let (pipeline_layout, base_pipeline) = match derive {
                | PipelineDeriveState::AsChildren { parent } => {
                    (parent.layout.handle, parent.handle)
                },
                | PipelineDeriveState::AsParent
                | PipelineDeriveState::Independence => {
                    let layout = config.content.layout_builder.build(&self.device)?;
                    (layout, vk::Pipeline::null())
                },
            };
            layouts.push(pipeline_layout);

            let graphics_pipeline_create_info = vk::GraphicsPipelineCreateInfo {
                s_type: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
                p_next: ptr::null(),
                flags : ci_flag.0,
                stage_count: shader_create_infos.len() as _,
                p_stages   : shader_create_infos.as_ptr(),
                p_vertex_input_state  : &config.content.states.vertex_input.info(),
                p_input_assembly_state: &config.content.states.input_assembly.info(),
                p_viewport_state      : &config.content.states.viewport.info(),
                p_rasterization_state : &config.content.states.rasterizer.info(),
                p_multisample_state   : &config.content.states.multisample.info(),
                p_depth_stencil_state : &config.content.states.depth_stencil.info(),
                p_color_blend_state   : &config.content.states.blend.info(),
                p_tessellation_state  : tessellation_info,
                p_dynamic_state       : dynamic_info,
                layout     : pipeline_layout,
                render_pass: config.content.render_pass.handle,
                // TODO: Add configuration for this field.
                subpass: 0,
                base_pipeline_handle: base_pipeline,
                base_pipeline_index: -1,
            };

            infos.push(graphics_pipeline_create_info);
            // Notice: keep `shader_create_infos` outlive for loop, or the pointer will be invalid.
            _shader_infos.push(shader_create_infos);
        }

        let handles = unsafe {
            self.device.handle.create_graphics_pipelines(vk::PipelineCache::null(), &infos, None)
                .or(Err(VkError::create("Graphics Pipelines")))?
        };

        let pipelines = self.configs.iter().enumerate()
            .map(|(i, config)|
                GsPipeline::new(&self.device, handles[i], layouts[i], config.content.render_pass.clone())
        ).collect();

        Ok(pipelines)
    }
}

impl<'a> Drop for GraphicsPipelineBuilder<'a> {

    fn drop(&mut self) {

        for config in self.configs.iter() {
            for module in config.modules.iter() {
                module.destroy(&self.device);
            }
        }
    }
}
// ------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------
pub enum PipelineDeriveState<'p> {
    AsParent,
    AsChildren { parent: &'p GsPipeline<Graphics> },
    Independence,
}

impl<'p> PipelineDeriveState<'p> {

    fn flag(&self) -> vk::PipelineCreateFlags {
        match self {
            | PipelineDeriveState::AsParent => vk::PipelineCreateFlags::ALLOW_DERIVATIVES,
            | PipelineDeriveState::AsChildren { .. } => vk::PipelineCreateFlags::DERIVATIVE,
            | PipelineDeriveState::Independence => vk::PipelineCreateFlags::empty(),
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

    fn combine_derive(&self, derive: &PipelineDeriveState) -> GsPipelineCIFlags {
        GsPipelineCIFlags(self.0 | derive.flag())
    }
    fn empty() -> GsPipelineCIFlags {
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
pub struct GraphicsPipelineConfig {

    shaders: Vec<GsShaderInfo>,
    states: PipelineStates,
    render_pass: GsRenderPass,

    layout_builder: PipelineLayoutBuilder,
}

impl GraphicsPipelineConfig {

    pub fn new(shaders: impl Into<Vec<GsShaderInfo>>, input: VertexInputDescription, render_pass: GsRenderPass, dimension: vkDim2D) -> GraphicsPipelineConfig {

        GraphicsPipelineConfig {
            shaders : shaders.into(),
            states  : PipelineStates::setup(input, dimension),

            render_pass,
            layout_builder: PipelineLayoutBuilder::default(),
        }
    }

    pub fn with_shader(mut self, shaders: Vec<GsShaderInfo>) -> GraphicsPipelineConfig {
        self.shaders = shaders;
        self
    }

    pub fn finish(self) -> GraphicsPipelineConfig {
        self
    }

    pub fn with_input_vertex(mut self, state: GsVertexInputState) -> GraphicsPipelineConfig {
        self.states.vertex_input = state;
        self
    }

    pub fn with_input_assembly(mut self, state: GsInputAssemblyState) -> GraphicsPipelineConfig {
        self.states.input_assembly = state;
        self
    }

    pub fn with_viewport(mut self, state: ViewportStateType) -> GraphicsPipelineConfig {

        match state {
            | ViewportStateType::Fixed { .. } => {},
            | ViewportStateType::Dynamic { .. } => {
                self.states.dynamic.add_state(vk::DynamicState::VIEWPORT);
                self.states.dynamic.add_state(vk::DynamicState::SCISSOR);
            },
            | ViewportStateType::DynamicViewportFixedScissor { .. } => {
                self.states.dynamic.add_state(vk::DynamicState::VIEWPORT);
            },
            | ViewportStateType::FixedViewportDynamicScissor { .. } => {
                self.states.dynamic.add_state(vk::DynamicState::SCISSOR);
            },
        }

        self.states.viewport = state.into_viewport_state();
        self
    }

    pub fn with_rasterizer(mut self, state: GsRasterizerState) -> GraphicsPipelineConfig {

        if state.is_dynamic_lindwidth() {
            self.states.dynamic.add_state(vk::DynamicState::LINE_WIDTH);
        }
        if state.is_dynamic_depthbias() {
            self.states.dynamic.add_state(vk::DynamicState::DEPTH_BIAS);
        }

        self.states.rasterizer = state;
        self
    }

    pub fn with_multisample(mut self, state: GsMultisampleState) -> GraphicsPipelineConfig {
        self.states.multisample = state;
        self
    }

    pub fn with_depth_stencil(mut self, state: GsDepthStencilState) -> GraphicsPipelineConfig {

        if state.depth.is_dynamic_depthbound() {
            self.states.dynamic.add_state(vk::DynamicState::DEPTH_BOUNDS);
        }
        if state.stencil.is_dynamic_compare_mask() {
            self.states.dynamic.add_state(vk::DynamicState::STENCIL_COMPARE_MASK);
        }
        if state.stencil.is_dynamic_write_mask() {
            self.states.dynamic.add_state(vk::DynamicState::STENCIL_WRITE_MASK);
        }
        if state.stencil.is_dynamic_reference() {
            self.states.dynamic.add_state(vk::DynamicState::STENCIL_REFERENCE);
        }

        self.states.depth_stencil = state;
        self
    }

    pub fn with_blend(mut self, state: GsBlendState) -> GraphicsPipelineConfig {

        if state.is_dynamic_blend_constants() {
            self.states.dynamic.add_state(vk::DynamicState::BLEND_CONSTANTS);
        }

        self.states.blend = state;
        self
    }

    pub fn with_tessellation(mut self, tessellation: GsTessellationState) -> GraphicsPipelineConfig {
        self.states.tessellation = Some(tessellation);
        self
    }

    pub fn add_descriptor_sets(mut self, sets: &[&DescriptorSet]) -> GraphicsPipelineConfig {

        for set in sets.into_iter() {
            self.layout_builder.add_descriptor_layout(&set.layout);
        }
        self
    }

    pub fn add_push_constants(mut self, ranges: Vec<GsPushConstantRange>) -> GraphicsPipelineConfig {

        for range in ranges.into_iter() {
            self.layout_builder.add_push_constant(range);
        }
        self
    }
}
// ------------------------------------------------------------------------------------------
