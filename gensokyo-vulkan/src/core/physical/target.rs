
use ash::vk;

use crate::core::physical::family::PhysicalQueueFamilies;
use crate::core::physical::property::PhysicalProperties;
use crate::core::physical::features::PhysicalFeatures;
use crate::core::physical::extension::PhysicalExtension;
use crate::core::physical::memory::PhysicalMemory;
use crate::core::physical::formats::PhysicalFormats;

pub struct GsPhysicalDevice {

    pub(crate) handle: vk::PhysicalDevice,

    pub(crate) properties: PhysicalProperties,
    pub(crate) families  : PhysicalQueueFamilies,
    pub(crate) features  : PhysicalFeatures,
    pub(crate) extensions: PhysicalExtension,
    pub(crate) memory    : PhysicalMemory,
    pub(crate) formats   : PhysicalFormats,
}

impl GsPhysicalDevice {

    pub fn limits(&self) -> &vk::PhysicalDeviceLimits {
        &self.properties.handle.limits
    }

    pub fn discard(&self) {
        // No method for delete physical device.
        // leave it empty
    }
}
