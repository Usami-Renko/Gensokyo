
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;

use crate::pipeline::shader::GsShaderModule;
use crate::pipeline::shader::shaderc::{ GsShaderCompiler, ShaderCompilePrefab, ShadercConfiguration };
use crate::pipeline::graphics::builder;
use crate::pipeline::graphics::builder::{ PipelineDeriveState, GsPipelineCIFlags };
use crate::pipeline::graphics::config::GfxPipelineConfig;
use crate::pipeline::target::GsPipeline;

use crate::error::{ VkResult, VkError };
use crate::utils::phantom::Graphics;

// ------------------------------------------------------------------------------------------
/// Graphic Multiply Pipeline Builder.
pub struct GfxMultiPipelineBuilder {

    device : GsDevice,
    ci_flag: GsPipelineCIFlags,
    configs: Vec<PipelineConfigTmp>,
    shaderc: GsShaderCompiler,
}

struct PipelineConfigTmp {
    content: GfxPipelineConfig,
    modules: Vec<GsShaderModule>,
}

impl GfxMultiPipelineBuilder {

    pub fn new(device: &GsDevice) -> VkResult<GfxMultiPipelineBuilder> {

        let builder = GfxMultiPipelineBuilder {
            device : device.clone(),
            ci_flag: GsPipelineCIFlags::default(),
            configs: vec![],
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

    pub fn add_config(&mut self, config: GfxPipelineConfig) -> VkResult<()> {

        let shader_modules = builder::compile_shaders(&self.device, &mut self.shaderc, &config.shaders)?;

        self.configs.push(PipelineConfigTmp {
            content: config,
            modules: shader_modules,
        });

        Ok(())
    }

    pub fn build(self) -> VkResult<Vec<GsPipeline<Graphics>>> {

        let pipeline_count = self.configs.len();
        let derive_state = PipelineDeriveState::Independence;

        let mut layouts = Vec::with_capacity(pipeline_count);
        let mut _shader_cis = Vec::with_capacity(pipeline_count);
        let mut pipeline_cis = Vec::with_capacity(pipeline_count);

        for config in self.configs.iter() {

            let pipeline_ci = builder::pipeline_ci(&self.device, &self.ci_flag, &config.modules, &config.content, &derive_state)?;

            pipeline_cis.push(pipeline_ci.content);
            layouts.push(pipeline_ci.pipeline_layout);
            // Notice: keep `shader_ci` outlive for loop, or the pointer will be invalid.
            _shader_cis.push(pipeline_ci.shader_cis);
        }

        let handles = unsafe {
            self.device.handle.create_graphics_pipelines(vk::PipelineCache::null(), &pipeline_cis, None)
                .or(Err(VkError::create("Graphics Pipelines")))?
        };
        self.destroy_modules();

        let mut pipelines = Vec::with_capacity(self.configs.len());
        let device_cloned = self.device.clone();
        for (i, config) in self.configs.into_iter().enumerate() {
            let pipeline = GsPipeline::new(device_cloned.clone(), handles[i], layouts[i].clone(), config.content.render_pass);
            pipelines.push(pipeline);
        }

        Ok(pipelines)
    }

    pub fn reset(&mut self) {
        self.ci_flag = GsPipelineCIFlags::default();
        self.configs.clear();
    }

    fn destroy_modules(&self) {
        for config in self.configs.iter() {
            builder::destroy_modules(&self.device, &config.modules);
        }
    }
}
// ------------------------------------------------------------------------------------------
