
use ash::vk;
use ash::version::InstanceV1_0;

use crate::core::instance::GsInstance;
use crate::core::physical::family::PhysicalQueueFamilies;
use crate::core::physical::features::PhyscialFeatures;
use crate::core::physical::extension::PhysicalExtension;
use crate::core::physical::memory::PhysicalMemory;

pub struct GsPhysicalDevice {

    pub(crate) handle: vk::PhysicalDevice,

    pub(crate) families: PhysicalQueueFamilies,
    pub(crate) features: PhyscialFeatures,
    pub(crate) extensions: PhysicalExtension,
    pub(crate) memory: PhysicalMemory,
}

impl GsPhysicalDevice {

    pub fn query_format_support(&self, instance: &GsInstance, format: vk::Format, query: &PhysicalFormatQueryContent) -> bool {

        let format_properties = unsafe {
            instance.handle.get_physical_device_format_properties(self.handle, format)
        };

        format_properties.linear_tiling_features.contains(query.linear_tiling) &&
            format_properties.optimal_tiling_features.contains(query.optimal_tiling) &&
            format_properties.buffer_features.contains(query.buffers)
    }

    pub fn cleanup(&self) {
        // No method for delete physical device
        // leave it empty
    }
}

pub struct PhysicalFormatQueryContent {

    pub linear_tiling: vk::FormatFeatureFlags,
    pub optimal_tiling: vk::FormatFeatureFlags,
    pub buffers: vk::FormatFeatureFlags,
}
