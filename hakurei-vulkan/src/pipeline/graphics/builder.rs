
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use pipeline::shader::{ HaShaderModule, HaShaderInfo, VertexInputDescription };
use pipeline::state::PipelineStates;
use pipeline::state::vertex_input::HaVertexInputState;
use pipeline::state::input_assembly::HaInputAssemblyState;
use pipeline::state::viewport::ViewportStateType;
use pipeline::state::rasterizer::HaRasterizerState;
use pipeline::state::multisample::HaMultisampleState;
use pipeline::state::depth_stencil::HaDepthStencilState;
use pipeline::state::blend::HaBlendState;
use pipeline::state::tessellation::HaTessellationState;
use pipeline::pass::HaRenderPass;
use pipeline::graphics::pipeline::{ HaGraphicsPipeline, GraphicsPipelineContainer };
use pipeline::layout::PipelineLayoutBuilder;
use pipeline::error::PipelineError;

use descriptor::ToDescriptorSetLayout;

use pipeline::shader::shaderc::{ HaShaderCompiler, ShaderCompilePrefab, ShadercConfiguration };

use types::vkuint;

use std::ptr;

pub struct GraphicsPipelineConfig {

    shaders        : Vec<HaShaderInfo>,
    states         : PipelineStates,
    render_pass    : Option<HaRenderPass>,
    flags          : vk::PipelineCreateFlags,

    shader_modules : Vec<HaShaderModule>,
    layout_builder : PipelineLayoutBuilder,
}

impl GraphicsPipelineConfig {

    pub fn new(shaders: impl Into<Vec<HaShaderInfo>>, input: VertexInputDescription, pass: HaRenderPass) -> GraphicsPipelineConfig {

        GraphicsPipelineConfig {
            shaders    : shaders.into(),
            states     : PipelineStates::setup(input),
            render_pass: Some(pass),
            flags      : vk::PipelineCreateFlags::empty(),

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

    device : HaDevice,
    configs: Vec<GraphicsPipelineConfig>,
    shaderc: HaShaderCompiler,
}

impl GraphicsPipelineBuilder {

    pub fn new(device: &HaDevice) -> Result<GraphicsPipelineBuilder, PipelineError> {

        let builder = GraphicsPipelineBuilder {
            device : device.clone(),
            configs: Vec::new(),
            shaderc: HaShaderCompiler::setup(ShaderCompilePrefab::Vulkan)?,
        };

        Ok(builder)
    }

    pub fn set_shaderc(&mut self, configuration: ShadercConfiguration) -> Result<(), PipelineError> {

        self.shaderc = HaShaderCompiler::setup_from_configuration(configuration)
            .map_err(|e| PipelineError::Shaderc(e))?;

        Ok(())
    }

    pub fn add_config(&mut self, config: GraphicsPipelineConfig) -> usize {

        let pipeline_index = self.configs.len();
        self.configs.push(config);

        pipeline_index
    }

    pub fn build(&mut self) -> Result<GraphicsPipelineContainer, PipelineError> {

        for config in self.configs.iter_mut() {
            let mut shader_modules = Vec::new();
            for shader in config.shaders.iter() {
                let module = shader.build(&self.device, &mut self.shaderc)?;
                shader_modules.push(module);
            }
            config.shader_modules = shader_modules;
        }

        let mut layouts = Vec::new();
        let mut infos = Vec::new();

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
                stage_count: shader_create_infos.len() as vkuint,
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
                render_pass: config.render_pass.as_ref().unwrap().handle,
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
            self.device.handle.create_graphics_pipelines(vk::PipelineCache::null(), infos.as_slice(), None).unwrap()
        };

        let mut pipelines = vec![];
        for (i, config) in self.configs.iter_mut().enumerate() {
            let render_pass = config.render_pass.take().unwrap(); // take ownership of HaRenderPass.
            let pipeline = HaGraphicsPipeline::new(&self.device, handles[i], layouts[i], render_pass);
            pipelines.push(pipeline);
        }

        self.clean_shader_modules();

        let container = GraphicsPipelineContainer::new(pipelines);
        Ok(container)
    }

    fn clean_shader_modules(&self) {

        self.configs.iter().for_each(|config| {
            config.shader_modules.iter().for_each(|module| {
                module.cleanup(&self.device);
            });
        });
    }
}


impl GraphicsPipelineConfig {

    pub fn resetup_shader(mut self, shaders: Vec<HaShaderInfo>) -> GraphicsPipelineConfig {
        self.shaders = shaders;
        self
    }

    pub fn setup_input_vertex(mut self, state: HaVertexInputState) -> GraphicsPipelineConfig {
        self.states.vertex_input = state;
        self
    }

    pub fn setup_input_assembly(mut self, state: HaInputAssemblyState) -> GraphicsPipelineConfig {
        self.states.input_assembly = state;
        self
    }

    pub fn setup_viewport(mut self, state: ViewportStateType) -> GraphicsPipelineConfig {

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

    pub fn setup_rasterizer(mut self, state: HaRasterizerState) -> GraphicsPipelineConfig {

        if state.is_dynamic_lindwidth() {
            self.states.dynamic.add_state(vk::DynamicState::LINE_WIDTH);
        }
        if state.is_dynamic_depthbias() {
            self.states.dynamic.add_state(vk::DynamicState::DEPTH_BIAS);
        }

        self.states.rasterizer = state;
        self
    }

    pub fn setup_multisample(mut self, state: HaMultisampleState) -> GraphicsPipelineConfig {
        self.states.multisample = state;
        self
    }

    pub fn setup_depth_stencil(mut self, state: HaDepthStencilState) -> GraphicsPipelineConfig {

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

    pub fn setup_blend(mut self, state: HaBlendState) -> GraphicsPipelineConfig {

        if state.is_dynamic_blend_constants() {
            self.states.dynamic.add_state(vk::DynamicState::BLEND_CONSTANTS);
        }

        self.states.blend = state;
        self
    }

    pub fn setup_tessllation(mut self, tessellation: HaTessellationState) -> GraphicsPipelineConfig {
        self.states.tessellation = Some(tessellation);
        self
    }

    pub fn add_descriptor_set(mut self, set: &impl ToDescriptorSetLayout) -> GraphicsPipelineConfig {
        self.layout_builder.add_descriptor_layout(&set.to_set_layout());
        self
    }
}
