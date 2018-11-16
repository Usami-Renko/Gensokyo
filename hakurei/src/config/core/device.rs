
use toml;

use config::engine::ConfigMirror;
use config::error::{ ConfigError, MappingError };

use vk::core::device::DeviceConfig;
use vk::core::physical::QueueOperationType;
use vk::core::physical::{ PhysicalDeviceType, PhysicalFeatureType, DeviceExtensionType };

use vk::core::device::QueueRequestStrategy;

use utils::time::TimePeriod;
use std::time::Duration;

#[derive(Deserialize, Default)]
pub(crate) struct DeviceConfigMirror {

    types     : Vec<String>,
    features  : Vec<String>,
    extensions: Vec<String>,
    queue_ops : Vec<String>,

    queue_request_strategy: String,
    transfer_time_out: String,
    transfer_duration: u64, // in ms unit
}

impl ConfigMirror for DeviceConfigMirror {
    type ConfigType = DeviceConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        let mut device_types = vec![];
        for raw_device_type in self.types.iter() {
            device_types.push(vk_raw2device_type(raw_device_type)?);
        }

        use vk::utils::format::vk_string_to_physical_feature;
        let mut features = vec![];
        for raw_feature in self.features.iter() {
            features.push(vk_string_to_physical_feature(raw_feature));
        }

        let mut extensions = vec![];
        for raw_extension in self.extensions.iter() {
            extensions.push(vk_raw2device_extension(raw_extension)?);
        }

        let mut queue_operations = vec![];
        for raw_operation in self.queue_ops.iter() {
            queue_operations.push(vk_raw2queue_operations(raw_operation)?);
        }

        let config = DeviceConfig {
            device_types, features, extensions, queue_operations,
            queue_request_strategy: vk_raw2queue_request_strategy(&self.queue_request_strategy)?,
            transfer_wait_time: vk_raw2transfer_wait_time(&self.transfer_time_out, self.transfer_duration)?.vulkan_time(),
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError> {

        if let Some(v) = toml.get("types") {
            if let Some(types) = v.as_array() {
                if types.len() > 0 {
                    self.types.clear();

                    for device_type in types {
                        let value = device_type.as_str().ok_or(ConfigError::ParseError)?;
                        self.types.push(value.to_owned());
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

        if let Some(v) = toml.get("queue_ops") {
            if let Some(queue_ops) = v.as_array() {
                if queue_ops.len() > 0 {
                    self.queue_ops.clear();

                    for op in queue_ops {
                        let value = op.as_str().ok_or(ConfigError::ParseError)?;
                        self.queue_ops.push(value.to_owned());
                    }
                }
            } else {
                return Err(ConfigError::ParseError);
            }
        }

        if let Some(v) = toml.get("queue_request_strategy") {
            self.queue_request_strategy = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
        }

        if let Some(v) = toml.get("transfer_time_out") {
            self.transfer_time_out = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
        }

        if let Some(v) = toml.get("transfer_duration") {
            self.transfer_duration = v.as_integer().ok_or(ConfigError::ParseError)? as u64;
        }

        Ok(())
    }
}

fn vk_raw2device_type(raw: &String) -> Result<PhysicalDeviceType, ConfigError> {

    let device_type = match raw.as_str() {
        | "DiscreteGPU"   => PhysicalDeviceType::DiscreteGPU,
        | "IntegratedGPU" => PhysicalDeviceType::IntegratedGPU,
        | "CPU"           => PhysicalDeviceType::CPU,
        | "VirtualGPU"    => PhysicalDeviceType::VirtualGPU,
        | _ => return Err(ConfigError::Mapping(MappingError::DeviceTypeError)),
    };

    Ok(device_type)
}

fn vk_raw2device_extension(raw: &String) -> Result<DeviceExtensionType, ConfigError> {

    let extension_type = match raw.as_str() {
        | "swapchain" => DeviceExtensionType::Swapchain,
        | _ => return Err(ConfigError::Mapping(MappingError::PhysicalExtensionError)),
    };

    Ok(extension_type)
}

fn vk_raw2queue_operations(raw: &String) -> Result<QueueOperationType, ConfigError> {

    let operation = match raw.as_str() {
        | "Graphics"      => QueueOperationType::Graphics,
        | "Compute"       => QueueOperationType::Compute,
        | "Transfer"      => QueueOperationType::Transfer,
        | "SparseBinding" => QueueOperationType::SparseBinding,
        | _ => return Err(ConfigError::Mapping(MappingError::DeviceQueueOperationError)),
    };

    Ok(operation)
}

fn vk_raw2queue_request_strategy(raw: &String) -> Result<QueueRequestStrategy, ConfigError> {

    let strategy = match raw.as_str() {
        | "SingleFamilySingleQueue" => QueueRequestStrategy::SingleFamilySingleQueue,
        | "SingleFamilyMultiQueues" => QueueRequestStrategy::SingleFamilyMultiQueues,
        | _ => return Err(ConfigError::Mapping(MappingError::QueueStrategyError)),
    };

    Ok(strategy)
}

fn vk_raw2transfer_wait_time(time_out: &String, duration: u64) -> Result<TimePeriod, ConfigError> {

    let time = match time_out.as_str() {
        | "Infinte"   => TimePeriod::Infinte,
        | "Immediate" => TimePeriod::Immediate,
        | "Timing"    => TimePeriod::Time(Duration::from_millis(duration)),
        | _ => return Err(ConfigError::Mapping(MappingError::DeviceTransferTimeError)),
    };

    Ok(time)
}
