
use ash::vk;
use ash::version::InstanceV1_0;

use crate::core::instance::GsInstance;
use crate::types::format::GsFormat;
use crate::error::{ VkResult, VkError };

use std::collections::HashMap;

pub(crate) struct PhysicalFormats {

    formats: HashMap<GsFormat, vk::FormatProperties>,
}

#[derive(Debug, Clone)]
pub struct PhysicalFormatsConfig {

    pub query_formats: Vec<GsFormat>,
}

impl PhysicalFormats {

    pub fn query(instance: &GsInstance, physical_device: vk::PhysicalDevice, config: &PhysicalFormatsConfig) -> PhysicalFormats {

        let mut formats = HashMap::with_capacity(config.query_formats.len());

        for &query_format in config.query_formats.iter() {
            let format_property = unsafe {
                instance.handle.get_physical_device_format_properties(physical_device, query_format.0)
            };
            formats.insert(query_format, format_property);
        }

        PhysicalFormats { formats }
    }

    pub fn query_format_linear(&self, format: GsFormat, query_linear: vk::FormatFeatureFlags) -> VkResult<bool> {

        let format_properties = self.query_format(format)?;
        Ok(format_properties.linear_tiling_features.contains(query_linear))
    }

    pub fn query_format_optimal(&self, format: GsFormat, query_optimal: vk::FormatFeatureFlags) -> VkResult<bool> {

        let format_properties = self.query_format(format)?;
        Ok(format_properties.optimal_tiling_features.contains(query_optimal))
    }

    #[allow(dead_code)]
    pub fn query_format_buffers(&self, format: GsFormat, query_buffers: vk::FormatFeatureFlags) -> VkResult<bool> {

        let format_properties = self.query_format(format)?;
        Ok(format_properties.buffer_features.contains(query_buffers))
    }

    fn query_format(&self, format: GsFormat) -> VkResult<&vk::FormatProperties> {

        self.formats.get(&format).ok_or(
            VkError::other(format!("Querying vk::Format: {:?} is not included in the config, please add it to [core.physical.query_formats]", format)))
    }
}
