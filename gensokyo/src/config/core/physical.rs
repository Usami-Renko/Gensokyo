
use toml;
use ash::vk;

use config::engine::ConfigMirror;
use config::error::{ ConfigError, MappingError };

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

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

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

    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError> {

        if let Some(v) = toml.get("extensions") {
            if let Some(extensions) = v.as_array() {
                if extensions.len() > 0 {
                    self.extensions.clear();

                    for extension in extensions {
                        let value = extension.as_str().ok_or(ConfigError::ParseError)?;
                        self.extensions.push(value.to_owned());
                    }
                }
            } else {
                return Err(ConfigError::ParseError);
            }
        }

        if let Some(v) = toml.get("queue_capabilities") {
            if let Some(queue_capalities) = v.as_array() {
                if queue_capalities.len() > 0 {
                    self.capabilities.clear();

                    for capalitity in queue_capalities {
                        let value = capalitity.as_str().ok_or(ConfigError::ParseError)?;
                        self.capabilities.push(value.to_owned());
                    }
                }
            } else {
                return Err(ConfigError::ParseError);
            }
        }

        if let Some(v) = toml.get("features") {
            if let Some(features) = v.as_array() {
                if features.len() > 0 {
                    self.features.clear();

                    for feature in features {
                        let value = feature.as_str().ok_or(ConfigError::ParseError)?;
                        self.features.push(value.to_owned());
                    }
                }
            } else {
                return Err(ConfigError::ParseError);
            }
        }

        if let Some(v) = toml.get("device_types") {
            if let Some(types) = v.as_array() {
                if types.len() > 0 {
                    self.devices.clear();

                    for device_type in types {
                        let value = device_type.as_str().ok_or(ConfigError::ParseError)?;
                        self.devices.push(value.to_owned());
                    }
                }
            } else {
                return Err(ConfigError::ParseError);
            }
        }

        Ok(())
    }
}

fn vk_raw2device_extension(raw: &String) -> Result<DeviceExtensionType, ConfigError> {

    let extension_type = match raw.as_str() {
        | "VK_KHR_swapchain"   => DeviceExtensionType::Swapchain,
        | _ => return Err(ConfigError::Mapping(MappingError::PhysicalExtensionError)),
    };

    Ok(extension_type)
}

fn vk_raw2queue_capability(raw: &String) -> Result<vk::QueueFlags, ConfigError> {

    let operation = match raw.as_str() {
        | "Graphics"      => vk::QueueFlags::GRAPHICS,
        | "Compute"       => vk::QueueFlags::COMPUTE,
        | "Transfer"      => vk::QueueFlags::TRANSFER,
        | "SparseBinding" => vk::QueueFlags::SPARSE_BINDING,
        | "Protected"     => vk::QueueFlags::PROTECTED,
        | _ => return Err(ConfigError::Mapping(MappingError::DeviceQueueOperationError)),
    };

    Ok(operation)
}

fn vk_raw2device_type(raw: &String) -> Result<vk::PhysicalDeviceType, ConfigError> {

    let device_type = match raw.as_str() {
        | "DiscreteGPU"   => vk::PhysicalDeviceType::DISCRETE_GPU,
        | "IntegratedGPU" => vk::PhysicalDeviceType::INTEGRATED_GPU,
        | "CPU"           => vk::PhysicalDeviceType::CPU,
        | "VirtualGPU"    => vk::PhysicalDeviceType::VIRTUAL_GPU,
        | _ => return Err(ConfigError::Mapping(MappingError::DeviceTypeError)),
    };

    Ok(device_type)
}

