
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;

use crate::pipeline::target::{ GsPipelineSet, PipelineIndex };
use crate::pipeline::shader::shaderc::{ GsShaderCompiler, ShaderCompilePrefab, ShadercConfiguration };
use crate::pipeline::graphics::builder;
use crate::pipeline::graphics::builder::{ PipelineDeriveState, GsPipelineCIFlags };
use crate::pipeline::graphics::config::GfxPipelineConfig;

use crate::error::{ VkResult, VkError };
use crate::utils::phantom::Graphics;


/// Graphics Pipeline Set Builder.
pub struct GfxPipelineSetBuilder {

    device : GsDevice,
    ci_flag: GsPipelineCIFlags,
    shaderc: GsShaderCompiler,

    template: GfxPipelineConfig,
    layout: vk::PipelineLayout,
    pipelines: Vec<vk::Pipeline>,

    is_use_base_pipeline: bool,
}

impl GfxPipelineSetBuilder {

    pub fn new(device: &GsDevice, template: GfxPipelineConfig) -> VkResult<GfxPipelineSetBuilder> {

        let layout = template.layout_builder.build(device)?;

        let builder = GfxPipelineSetBuilder {
            device : device.clone(),
            ci_flag: GsPipelineCIFlags::default(),
            shaderc: GsShaderCompiler::setup(ShaderCompilePrefab::Vulkan)?,
            template, layout,
            pipelines: vec![],
            is_use_base_pipeline: true,
        };
        Ok(builder)
    }

    pub fn with_flag(&mut self, flags: GsPipelineCIFlags) {
        self.ci_flag |= flags;
    }

    pub fn set_base_pipeline_use(&mut self, is_use_base_pipeline: bool) {
        self.is_use_base_pipeline = is_use_base_pipeline;
    }

    pub fn set_shaderc(&mut self, configuration: ShadercConfiguration) -> VkResult<()> {

        self.shaderc = GsShaderCompiler::from_configuration(configuration)?;

        Ok(())
    }

    pub fn template_mut(&mut self) -> &mut GfxPipelineConfig {
        &mut self.template
    }

    pub fn build_template(&mut self) -> VkResult<PipelineIndex> {

        let derive_state = if self.is_use_base_pipeline {
            if self.pipelines.is_empty() {
                PipelineDeriveState::AsParent {
                    layout: self.layout,
                }
            } else {
                PipelineDeriveState::AsChildren {
                    parent: self.pipelines.first().unwrap().clone(),
                    layout: self.layout,
                }
            }
        } else {
            PipelineDeriveState::IndependenceLayoutDefined { layout: self.layout }
        };

        // compile shader.
        let shader_modules = builder::compile_shaders(&self.device, &mut self.shaderc, &self.template.shaders)?;
        // generate create info.
        let pipeline_ci = builder::pipeline_ci(&self.device, &self.ci_flag, &shader_modules, &self.template, &derive_state)?;

        // build pipeline.
        let mut handles = unsafe {
            self.device.handle.create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_ci.content], None)
                .or(Err(VkError::create("Graphics Pipelines")))?
        };
        builder::destroy_modules(&self.device, &shader_modules);

        let index = PipelineIndex(self.pipelines.len());
        self.pipelines.push(handles.pop().unwrap());

        Ok(index)
    }

    pub fn collect_into_set(self) -> GsPipelineSet<Graphics> {

        GsPipelineSet::<Graphics>::new(self.device, self.pipelines, self.layout, self.template.render_pass)
    }
}
