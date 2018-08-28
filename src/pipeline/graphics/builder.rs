
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use pipeline::{
    graphics::pipeline::HaGraphicsPipeline,

    shader::module::{ HaShaderModule, HaShaderInfo },
    shader::input::VertexInputDescription,
    state::PipelineStates,
    state::vertex_input::HaVertexInput,
    state::input_assembly::HaInputAssembly,
    state::viewport::HaViewport,
    state::rasterizer::HaRasterizer,
    state::multisample::HaMultisample,
    state::depth_stencil::HaDepthStencil,
    state::blend::blending::HaBlend,
    state::tessellation::HaTessellation,
    state::dynamic::HaDynamicState,
    pass::render_pass::HaRenderPass,
    layout::PipelineLayoutBuilder,
    error::PipelineError,
};

use utility::marker::VulkanFlags;

use std::ptr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PipelineCreateFlag {
    /// DisableOptimizationBit specifies that the created pipeline will not be optimized.
    ///
    /// Using this flag may reduce the time taken to create the pipeline.
    DisableOptimizationBit,
    /// AllowDerivativesBit specifies that the pipeline to be created is allowed to be the parent of a pipeline
    /// that will be created in a subsequent call to vkCreateGraphicsPipelines or vkCreateComputePipelines.
    AllowDerivativesBit,
    /// DerivativeBit specifies that the pipeline to be created will be a child of a previously created parent pipeline.
    DerivativeBit,
    // TODO: Others flags are not supported in ash yet.
    // ViewIndexFromDeviceIndexBit specifies that any shader input variables decorated as ViewIndex will be assigned values
    // as if they were decorated as DeviceIndex.
    //ViewIndexFromDeviceIndexBit,
    // Same as ViewIndexFromDeviceIndexBit.
    //ViewIndexFromDeviceIndexBitKHR,
    // specifies that a compute pipeline can be used with vkCmdDispatchBase with a non-zero base workgroup.
    //DispatchBase,
    // Same as DispatchBase,
    //DispatchBaseKHR,
}
impl VulkanFlags for [PipelineCreateFlag] {
    type FlagType = vk::PipelineCreateFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::PipelineCreateFlags::empty(), |acc, flag| {
            match *flag {
                | PipelineCreateFlag::DisableOptimizationBit         => acc | vk::PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT,
                | PipelineCreateFlag::AllowDerivativesBit            => acc | vk::PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT,
                | PipelineCreateFlag::DerivativeBit                  => acc | vk::PIPELINE_CREATE_DERIVATIVE_BIT,
                //| PipelineCreateFlag::ViewIndexFromDeviceIndexBit    => acc | vk::PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT,
                //| PipelineCreateFlag::ViewIndexFromDeviceIndexBitKHR => acc | vk::PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR,
                //| PipelineCreateFlag::DispatchBase                   => acc | vk::PIPELINE_CREATE_DISPATCH_BASE,
                //| PipelineCreateFlag::DispatchBaseKHR                => acc | vk::PIPELINE_CREATE_DISPATCH_BASE_KHR,
            }
        })
    }
}

pub struct GraphicsPipelineConfig {

    shaders        : Vec<HaShaderInfo>,
    states         : PipelineStates,
    render_pass    : Option<HaRenderPass>,

    shader_modules : Vec<HaShaderModule>,
    layout_builder : PipelineLayoutBuilder,
}

impl GraphicsPipelineConfig {

    pub fn init(shaders: Vec<HaShaderInfo>, input: VertexInputDescription, pass: HaRenderPass) -> GraphicsPipelineConfig {

        GraphicsPipelineConfig {
            shaders,
            states         : PipelineStates::setup(input),
            render_pass    : Some(pass),

            shader_modules : vec![],
            layout_builder : PipelineLayoutBuilder::init(),
        }
    }

    pub fn finish_config(&mut self) {
        // TODO: Configure layout property here
        // code goes here...
    }
}

pub struct GraphicsPipelineBuilder {

    configs: Vec<GraphicsPipelineConfig>,
}

impl GraphicsPipelineBuilder {

    pub fn init() -> GraphicsPipelineBuilder {
        GraphicsPipelineBuilder {
            configs: vec![],
        }
    }
    pub fn add_config(&mut self, config: GraphicsPipelineConfig) {
        self.configs.push(config);
    }

    pub fn build(&mut self, device: &HaLogicalDevice) -> Result<Vec<HaGraphicsPipeline>, PipelineError> {

        for config in self.configs.iter_mut() {
            let mut shader_modules = vec![];
            for shader in config.shaders.iter() {
                let module = shader.build(device).map_err(|e| PipelineError::Shader(e))?;
                shader_modules.push(module);
            }
            config.shader_modules = shader_modules;
        }

        let mut layouts = vec![];
        let mut infos   = vec![];

        for config in self.configs.iter() {

            let shader_create_infos: Vec<vk::PipelineShaderStageCreateInfo> = config.shader_modules.iter()
                .map(|m| m.info().clone()).collect();
            let tessellation_info = config.states.tessellation.as_ref()
                .map_or(ptr::null(), |t| &t.info());
            let dynamic_info = config.states.dynamic.as_ref()
                .map_or(ptr::null(), |d| &d.info());

            let pipeline_layout = config.layout_builder.build(device)?;
            layouts.push(pipeline_layout);

            let graphics_pipeline_create_info = vk::GraphicsPipelineCreateInfo {
                s_type: vk::StructureType::GraphicsPipelineCreateInfo,
                p_next: ptr::null(),
                // TODO: Add configuration for flags
                flags: vk::PipelineCreateFlags::empty(),
                stage_count : shader_create_infos.len() as uint32_t,
                p_stages    : shader_create_infos.as_ptr(),
                p_vertex_input_state   : &config.states.vertex_input.info(),
                p_input_assembly_state : &config.states.input_assembly.info(),
                p_viewport_state       : &config.states.viewport.info(),
                p_rasterization_state  : &config.states.rasterizer.info(),
                p_multisample_state    : &config.states.multisample.info(),
                p_depth_stencil_state  : &config.states.depth_stencil.info(),
                p_color_blend_state    : &config.states.blend.info(),
                p_tessellation_state   : tessellation_info,
                p_dynamic_state        : dynamic_info,
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
            device.handle.create_graphics_pipelines(vk::PipelineCache::null(), infos.as_slice(), None).unwrap()
        };

        let mut pipelines = vec![];
        for (i, config) in self.configs.iter_mut().enumerate() {
            let render_pass = config.render_pass.take().unwrap(); // transfer ownership of HaRenderPass.
            let pipeline = HaGraphicsPipeline::new(handles[i], layouts[i], render_pass);
            pipelines.push(pipeline);

        }

        self.clean_shader_modules(device);

        Ok(pipelines)
    }

    fn clean_shader_modules(&self, device: &HaLogicalDevice) {
        self.configs.iter().for_each(|config| {
            config.shader_modules.iter().for_each(|module| {
                module.cleanup(device);
            });
        });
    }
}


impl GraphicsPipelineConfig {

    pub fn resetup_shader(&mut self, shaders: Vec<HaShaderInfo>) { self.shaders = shaders; }
    pub fn setup_input_vertex(&mut self, vertex_infos: HaVertexInput) { self.states.vertex_input = vertex_infos; }
    pub fn setup_input_assembly(&mut self, assembly: HaInputAssembly) { self.states.input_assembly = assembly; }
    pub fn setup_viewport(&mut self, viewport: HaViewport) { self.states.viewport = viewport; }
    pub fn setup_rasterizer(&mut self, rasterizer: HaRasterizer) { self.states.rasterizer = rasterizer; }
    pub fn setup_multisample(&mut self, multisample: HaMultisample) { self.states.multisample = multisample; }
    pub fn setup_depth_stencil(&mut self, depth_stencil: HaDepthStencil) { self.states.depth_stencil = depth_stencil; }
    pub fn setup_blend(&mut self, blend: HaBlend) { self.states.blend = blend; }
    pub fn setup_tessllation(&mut self, tessellation: HaTessellation) { self.states.tessellation = Some(tessellation); }
    pub fn setup_dynamic(&mut self, dynamic_state: HaDynamicState) { self.states.dynamic = Some(dynamic_state); }
}
