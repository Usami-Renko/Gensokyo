
use crate::core::physical::extension::PhysicalExtensionConfig;
use crate::core::physical::family::PhysicalQueueFamilyConfig;
use crate::core::physical::features::PhysicalFeatureConfig;
use crate::core::physical::property::PhysicalPropertiesConfig;
use crate::core::physical::formats::PhysicalFormatsConfig;

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
    pub formats      : PhysicalFormatsConfig,
}
