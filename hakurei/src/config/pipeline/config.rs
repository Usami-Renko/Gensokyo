
use config::pipeline::DepthStencilConfig;

pub struct PipelineConfig {

    pub depth_stencil: DepthStencilConfig,
}

impl Default for PipelineConfig {

    fn default() -> PipelineConfig {

        PipelineConfig {

            depth_stencil: DepthStencilConfig::default(),
        }
    }
}
