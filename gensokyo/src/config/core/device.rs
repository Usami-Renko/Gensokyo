
use toml;

use crate::config::engine::ConfigMirror;
use crate::config::error::{ ConfigError, MappingError };

use gsvk::core::device::DeviceConfig;
use gsvk::core::device::QueueRequestStrategy;

use crate::utils::time::TimePeriod;

use std::time::Duration;

#[derive(Deserialize, Default)]
pub(crate) struct DeviceConfigMirror {

    queue_request_strategy: String,
    transfer_time_out: String,
    transfer_duration: u64, // in ms unit
}

impl ConfigMirror for DeviceConfigMirror {
    type ConfigType = DeviceConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        let config = DeviceConfig {
            queue_request_strategy: vk_raw2queue_request_strategy(&self.queue_request_strategy)?,
            transfer_wait_time    : vk_raw2transfer_wait_time(&self.transfer_time_out, self.transfer_duration)?.vulkan_time(),
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError> {

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
