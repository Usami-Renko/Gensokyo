
use config::core::CoreConfig;
use config::window::WindowConfig;
use config::resources::ResourceConfig;

pub struct EngineConfig {

    pub core      : CoreConfig,
    pub window    : WindowConfig,
    pub resources : ResourceConfig,
}

impl Default for EngineConfig {

    fn default() -> EngineConfig {
        EngineConfig {
            core      : CoreConfig::default(),
            window    : WindowConfig::default(),
            resources : ResourceConfig::default(),
        }
    }
}
