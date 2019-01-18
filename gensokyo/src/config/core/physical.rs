
use toml;
use ash::vk;

use crate::config::engine::ConfigMirror;
use crate::error::{ GsResult, GsError };

use gsvk::core::physical::PhysicalConfig;
use gsvk::core::physical::DeviceExtensionType;
use gsvk::core::physical::{ PhysicalExtensionConfig, PhysicalQueueFamilyConfig, PhysicalFeatureConfig, PhysicalPropertiesConfig };

#[derive(Deserialize, Default)]
pub(crate) struct PhysicalConfigMirror {

    extensions   : Vec<String>,
    capabilities : Vec<String>,
    features     : Vec<String>,
    devices      : Vec<String>,
}

impl ConfigMirror for PhysicalConfigMirror {
    type ConfigType = PhysicalConfig;

    fn into_config(self) -> GsResult<Self::ConfigType> {

        let mut require_extensions = vec![];
        for raw_extension in self.extensions.iter() {
            require_extensions.push(vk_raw2device_extension(raw_extension)?);
        }

        let mut require_capabilities = vec![];
        for raw_capability in self.capabilities.iter() {
            require_capabilities.push(vk_raw2queue_capability(raw_capability)?);
        }

        use gsvk::utils::format::vk_string_to_physical_feature;
        let mut require_features = vk::PhysicalDeviceFeatures::default();
        for raw_feature in self.features.iter() {
            vk_string_to_physical_feature(raw_feature, &mut require_features);
        }

        let mut require_device_types = vec![];
        for raw_device_type in self.devices.iter() {
            require_device_types.push(vk_raw2device_type(raw_device_type)?);
        }

        let config = PhysicalConfig {
            extension    : PhysicalExtensionConfig { require_extensions },
            queue_family : PhysicalQueueFamilyConfig { require_capabilities },
            features     : PhysicalFeatureConfig { require_features },
            properties   : PhysicalPropertiesConfig { require_device_types },
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> GsResult<()> {

        if let Some(v) = toml.get("extensions") {
            if let Some(extensions) = v.as_array() {
                if extensions.len() > 0 {
                    self.extensions.clear();

                    for (i, extension) in extensions.iter().enumerate() {
                        let value = extension.as_str()
                            .ok_or(GsError::config(format!("extension #{}", i)))?;
                        self.extensions.push(value.to_owned());
                    }
                }
            } else {
                return Err(GsError::config("[core.physical.extensions]"))
            }
        }

        if let Some(v) = toml.get("queue_capabilities") {
            if let Some(queue_capabilities) = v.as_array() {
                if queue_capabilities.len() > 0 {
                    self.capabilities.clear();

                    for (i, capability) in queue_capabilities.iter().enumerate() {
                        let value = capability.as_str()
                            .ok_or(GsError::config(format!("capability #{}", i)))?;
                        self.capabilities.push(value.to_owned());
                    }
                }
            } else {
                return Err(GsError::config("[core.physical.queue_capabilities]"))
            }
        }

        if let Some(v) = toml.get("features") {
            if let Some(features) = v.as_array() {
                if features.len() > 0 {
                    self.features.clear();

                    for (i, feature) in features.iter().enumerate() {
                        let value = feature.as_str()
                            .ok_or(GsError::config(format!("features #{}", i)))?;
                        self.features.push(value.to_owned());
                    }
                }
            } else {
                return Err(GsError::config("[core.physical.features]"))
            }
        }

        if let Some(v) = toml.get("device_types") {
            if let Some(types) = v.as_array() {
                if types.len() > 0 {
                    self.devices.clear();

                    for (i, device_type) in types.iter().enumerate() {
                        let value = device_type.as_str()
                            .ok_or(GsError::config(format!("device_types #{}", i)))?;
                        self.devices.push(value.to_owned());
                    }
                }
            } else {
                return Err(GsError::config("[core.physical.device_types]"))
            }
        }

        Ok(())
    }
}

fn vk_raw2device_extension(raw: &String) -> GsResult<DeviceExtensionType> {

    let extension_type = match raw.as_str() {
        | "VK_KHR_swapchain" => DeviceExtensionType::Swapchain,
        | _ => return Err(GsError::config(raw)),
    };

    Ok(extension_type)
}

fn vk_raw2queue_capability(raw: &String) -> GsResult<vk::QueueFlags> {

    let operation = match raw.as_str() {
        | "Graphics"      => vk::QueueFlags::GRAPHICS,
        | "Compute"       => vk::QueueFlags::COMPUTE,
        | "Transfer"      => vk::QueueFlags::TRANSFER,
        | "SparseBinding" => vk::QueueFlags::SPARSE_BINDING,
        | "Protected"     => vk::QueueFlags::PROTECTED,
        | _ => return Err(GsError::config(raw)),
    };

    Ok(operation)
}

fn vk_raw2device_type(raw: &String) -> GsResult<vk::PhysicalDeviceType> {

    let device_type = match raw.as_str() {
        | "DiscreteGPU"   => vk::PhysicalDeviceType::DISCRETE_GPU,
        | "IntegratedGPU" => vk::PhysicalDeviceType::INTEGRATED_GPU,
        | "CPU"           => vk::PhysicalDeviceType::CPU,
        | "VirtualGPU"    => vk::PhysicalDeviceType::VIRTUAL_GPU,
        | _ => return Err(GsError::config(raw)),
    };

    Ok(device_type)
}

