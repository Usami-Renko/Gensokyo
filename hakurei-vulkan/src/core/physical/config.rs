
use core::physical::extension::PhysicalExtensionConfig;
use core::physical::family::PhysicalQueueFamilyConfig;
use core::physical::features::PhysicalFeatureConfig;
use core::physical::property::PhysicalPropertiesConfig;

pub(crate) trait PhysicalInspectProperty {
    type ConfigType;

    fn inspect(&self, config: &Self::ConfigType) -> bool;
    fn set(&mut self, config: &Self::ConfigType);
}

#[derive(Debug, Clone)]
pub struct PhysicalConfig {

    pub extension    : PhysicalExtensionConfig,
    pub queue_family : PhysicalQueueFamilyConfig,
    pub features     : PhysicalFeatureConfig,
    pub properties   : PhysicalPropertiesConfig,
}
