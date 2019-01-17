
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

use std::ptr;

pub struct GraphicsPipelineConfig {

    shaders: Vec<GsShaderInfo>,
    states: PipelineStates,
    flags: vk::PipelineCreateFlags,
    render_pass: GsRenderPass,

    shader_modules: Vec<GsShaderModule>,
    layout_builder: PipelineLayoutBuilder,
}

impl GraphicsPipelineConfig {

    pub fn new(shaders: impl Into<Vec<GsShaderInfo>>, input: VertexInputDescription, render_pass: GsRenderPass, dimension: vkDim2D) -> GraphicsPipelineConfig {

        GraphicsPipelineConfig {
            shaders : shaders.into(),
            states  : PipelineStates::setup(input, dimension),
            flags   : vk::PipelineCreateFlags::empty(),

            render_pass,
            shader_modules: vec![],
            layout_builder: PipelineLayoutBuilder::default(),
        }
    }

    pub fn with_flags(&mut self, flags: vk::PipelineCreateFlags) {
        self.flags = flags;
    }

    pub fn finish(self) -> GraphicsPipelineConfig {
        // TODO: Configure layout property here
        // code goes here...

        self
    }
}

pub struct GraphicsPipelineBuilder {

    device : GsDevice,
    configs: Vec<GraphicsPipelineConfig>,
    shaderc: GsShaderCompiler,
}

impl GraphicsPipelineBuilder {

    pub fn new(device: &GsDevice) -> VkResult<GraphicsPipelineBuilder> {

        let builder = GraphicsPipelineBuilder {
            device : device.clone(),
            configs: vec![],
            shaderc: GsShaderCompiler::setup(ShaderCompilePrefab::Vulkan)?,
        };

        Ok(builder)
    }

    pub fn set_shaderc(&mut self, configuration: ShadercConfiguration) -> VkResult<()> {

        self.shaderc = GsShaderCompiler::setup_from_configuration(configuration)?;
        Ok(())
    }

    pub fn add_config(&mut self, config: GraphicsPipelineConfig) {

        self.configs.push(config);
    }

    pub fn build(mut self) -> VkResult<Vec<GsPipeline<Graphics>>> {

        for config in self.configs.iter_mut() {
            let mut shader_modules = vec![];
            for shader in config.shaders.iter() {
                let module = shader.build(&self.device, &mut self.shaderc)?;
                shader_modules.push(module);
            }
            config.shader_modules = shader_modules;
        }

        let mut layouts = vec![];
        let mut infos = vec![];

        for config in self.configs.iter() {

            let shader_create_infos: Vec<vk::PipelineShaderStageCreateInfo> = config.shader_modules.iter()
                .map(|m| m.info()).collect();
            let tessellation_info = config.states.tessellation.as_ref()
                .map_or(ptr::null(), |t| &t.info());
            let dynamic_info = if config.states.dynamic.is_contain_state() {
                &config.states.dynamic.info() } else { ptr::null() };

            let pipeline_layout = config.layout_builder.build(&self.device)?;
            layouts.push(pipeline_layout);

            let graphics_pipeline_create_info = vk::GraphicsPipelineCreateInfo {
                s_type: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
                p_next: ptr::null(),
                // TODO: Add configuration for vk::PipelineCreateFlags.
                flags : vk::PipelineCreateFlags::empty(),
                stage_count: shader_create_infos.len() as _,
                p_stages   : shader_create_infos.as_ptr(),
                p_vertex_input_state  : &config.states.vertex_input.info(),
                p_input_assembly_state: &config.states.input_assembly.info(),
                p_viewport_state      : &config.states.viewport.info(),
                p_rasterization_state : &config.states.rasterizer.info(),
                p_multisample_state   : &config.states.multisample.info(),
                p_depth_stencil_state : &config.states.depth_stencil.info(),
                p_color_blend_state   : &config.states.blend.info(),
                p_tessellation_state  : tessellation_info,
                p_dynamic_state       : dynamic_info,
                layout: pipeline_layout,
                render_pass: config.render_pass.handle,
                // TODO: Add configuration for this field.
                subpass: 0,
                // TODO: Add configuration for this field.
                base_pipeline_handle: vk::Pipeline::null(),
                // TODO: Add configuration for this field.
                base_pipeline_index: -1,
            };

            infos.push(graphics_pipeline_create_info);
        }

        let handles = unsafe {
            self.device.handle.create_graphics_pipelines(vk::PipelineCache::null(), infos.as_slice(), None)
                .or(Err(VkError::create("Graphics Pipelines")))?
        };

        self.destroy_shader_modules();

        let mut pipelines = vec![];
        for (i, config) in self.configs.into_iter().enumerate() {
            let pipeline = GsPipeline::new(&self.device, handles[i], layouts[i], config.render_pass);
            pipelines.push(pipeline);
        }

        Ok(pipelines)
    }

    fn destroy_shader_modules(&self) {

        self.configs.iter().for_each(|config| {
            config.shader_modules.iter().for_each(|module| {
                module.destroy(&self.device);
            });
        });
    }
}


impl GraphicsPipelineConfig {

    pub fn with_shader(mut self, shaders: Vec<GsShaderInfo>) -> GraphicsPipelineConfig {
        self.shaders = shaders;
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

    pub fn add_descriptor_set(mut self, set: &DescriptorSet) -> GraphicsPipelineConfig {
        self.layout_builder.add_descriptor_layout(&set.layout);
        self
    }

    pub fn add_push_constants(mut self, range: GsPushConstantRange) -> GraphicsPipelineConfig {
        self.layout_builder.add_push_constant(range);
        self
    }
}
