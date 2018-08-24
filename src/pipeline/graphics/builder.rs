
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use pipeline::{
    graphics::GraphicsPipeline,

    shader::{ HaShaderInfo, HaShaderModule },
    input_assembly::HaInputAssembly,
    tessellation::HaTessellationState,
    viewport::HaViewport,
    rasterizer::HaRasterizer,
    multisample::HaMultisample,
    depth_stencil::HaDepthStencil,
    blend::HaBlend,
    dynamic::HaDynamicState,
    pass::HaRenderPass,
    layout::{ PipelineLayoutBuilder, HaPipelineLayout },
    error::PipelineError,
};

use utility::marker::VulkanFlags;

use std::ptr;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PipelineCreateFlag {
    /// DisableOptimizationBit specifies that the created pipeline will not be optimized.
    ///
    /// Using this flag may reduce the time taken to create the pipeline.
    DisableOptimizationBit,
    /// AllowDerivativesBit specifies that the pipeline to be created is allowed to be the parent of a pipeline that will be created in a subsequent call to vkCreateGraphicsPipelines or vkCreateComputePipelines.
    AllowDerivativesBit,
    /// DerivativeBit specifies that the pipeline to be created will be a child of a previously created parent pipeline.
    DerivativeBit,
    // TODO: Others flags are not supported in ash yet.
    // ViewIndexFromDeviceIndexBit specifies that any shader input variables decorated as ViewIndex will be assigned values as if they were decorated as DeviceIndex.
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
    inputs         : HaInputAssembly,
    tessellation   : Option<HaTessellationState>,
    viewport       : Option<HaViewport>,
    rasterizer     : Option<HaRasterizer>,
    multisample    : Option<HaMultisample>,
    depth_stencil  : Option<HaDepthStencil>,
    blend          : Option<HaBlend>,
    dynamic        : Option<HaDynamicState>,
    render_pass    : Option<HaRenderPass>,

    shader_modules : Vec<HaShaderModule>,
    layout_builder : PipelineLayoutBuilder,
}

impl GraphicsPipelineConfig {

    pub fn init(shaders: Vec<HaShaderInfo>, inputs: HaInputAssembly, pass: HaRenderPass) -> GraphicsPipelineConfig {

        GraphicsPipelineConfig {
            shaders,
            inputs,
            tessellation   : None,
            viewport       : None,
            rasterizer     : None,
            multisample    : None,
            depth_stencil  : None,
            blend          : None,
            dynamic        : None,
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

    pub fn build(&mut self, device: &HaLogicalDevice) -> Result<Vec<GraphicsPipeline>, PipelineError> {

        for config in self.configs.iter_mut() {
            let mut shader_modules = vec![];
            for shader in config.shaders.iter() {
                let module = shader.build(device).map_err(|e| PipelineError::Shader(e))?;
                shader_modules.push(module);
            }
            config.shader_modules = shader_modules;
        }

        let mut layouts = vec![];
        let mut infos = vec![];

        for config in self.configs.iter() {

            let shader_create_infos: Vec<vk::PipelineShaderStageCreateInfo> = config.shader_modules.iter()
                .map(|m| m.info().clone()).collect();
            let viewport_info = config.viewport.as_ref()
                .map_or(HaViewport::init().info(), |v| v.info());
            let rasterization_info = config.rasterizer.as_ref()
                .map_or(HaRasterizer::init().info(), |r| r.info());
            let multisample_info = config.multisample.as_ref()
                .map_or(HaMultisample::init().info(), |m| m.info());
            let depth_stencil_info = config.depth_stencil.as_ref()
                .map_or(HaDepthStencil::init().info(), |d| d.info());
            let blend_info = config.blend.as_ref()
                .map_or(HaBlend::init().info(), |b| b.info());
            let tessellation_info = config.tessellation.as_ref()
                .map_or(ptr::null(), |t| &t.info());
            let dynamic_info = config.dynamic.as_ref()
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
                p_vertex_input_state   : &config.inputs.state,
                p_input_assembly_state : &config.inputs.assembly,
                p_viewport_state       : &viewport_info,
                p_rasterization_state  : &rasterization_info,
                p_multisample_state    : &multisample_info,
                p_depth_stencil_state  : &depth_stencil_info,
                p_color_blend_state    : &blend_info,
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
            let pipeline = GraphicsPipeline {
                handle: handles[i],
                layout: HaPipelineLayout::new(layouts[i]),
                pass:   render_pass,
            };
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

    #[allow(dead_code)]
    pub fn resetup_shader(&mut self, shaders: Vec<HaShaderInfo>) {
        self.shaders = shaders;
    }
    #[allow(dead_code)]
    pub fn resetup_input(&mut self, inputs: HaInputAssembly) {
        self.inputs = inputs;
    }
    #[allow(dead_code)]
    pub fn setup_tessllation(&mut self, tessellation: HaTessellationState) {
        self.tessellation = Some(tessellation);
    }
    #[allow(dead_code)]
    pub fn setup_viewport(&mut self, viewport: HaViewport) {
        self.viewport = Some(viewport);
    }
    #[allow(dead_code)]
    pub fn setup_rasterizer(&mut self, rasterizer: HaRasterizer) {
        self.rasterizer = Some(rasterizer);
    }
    #[allow(dead_code)]
    pub fn setup_multisample(&mut self, multisample: HaMultisample) {
        self.multisample = Some(multisample);
    }
    #[allow(dead_code)]
    pub fn setup_depth_stencil(&mut self, depth_stencil: HaDepthStencil) {
        self.depth_stencil = Some(depth_stencil);
    }
    #[allow(dead_code)]
    pub fn setup_blend(&mut self, blend: HaBlend) {
        self.blend = Some(blend);
    }
    #[allow(dead_code)]
    pub fn setup_dynamic_state(&mut self, dynamic_state: HaDynamicState) {
        self.dynamic = Some(dynamic_state);
    }
}
