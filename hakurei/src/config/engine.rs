
use config::core::CoreConfig;
use config::window::WindowConfig;
use config::pipeline::PipelineConfig;
use config::resources::ResourceConfig;

pub struct EngineConfig {

    pub core      : CoreConfig,
    pub window    : WindowConfig,
    pub pipeline  : PipelineConfig,
    pub resources : ResourceConfig,
}

impl Default for EngineConfig {

    fn default() -> EngineConfig {
        EngineConfig {
            core      : CoreConfig::default(),
            window    : WindowConfig::default(),
            pipeline  : PipelineConfig::default(),
            resources : ResourceConfig::default(),
        }
    }
}
