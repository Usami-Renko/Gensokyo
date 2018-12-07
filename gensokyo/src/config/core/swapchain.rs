
use toml;
use ash::vk;

use gsvk::core::swapchain::SwapchainConfig;

use gsvk::types::vkuint;

use crate::config::engine::ConfigMirror;
use crate::config::error::{ ConfigError, MappingError };

use crate::utils::time::TimePeriod;
use std::time::Duration;

#[derive(Deserialize, Default)]
pub(crate) struct SwapchainConfigMirror {
    
    image_count: vkuint,
    framebuffer_layers: vkuint,
    prefer_surface_format     : String,
    prefer_surface_color_space: String,
    present_mode_primary  : String,
    present_mode_secondary: String,
    acquire_image_time_out: String,
    acquire_image_duration: u64,
}

impl ConfigMirror for SwapchainConfigMirror {
    type ConfigType = SwapchainConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        use gsvk::utils::format::vk_string_to_format;

        let config = SwapchainConfig {
            image_count: self.image_count,
            framebuffer_layers: self.framebuffer_layers,

            prefer_surface_format     : vk_string_to_format(&self.prefer_surface_format),
            prefer_surface_color_space: vk_raw2colorspace(&self.prefer_surface_color_space)?,

            prefer_primary_present_mode  : vk_raw2presentmode(&self.present_mode_primary)?,
            prefer_secondary_present_mode: vk_raw2presentmode(&self.present_mode_secondary)?,

            acquire_image_time_out: vk_raw2acquire_image_time(&self.acquire_image_time_out, self.acquire_image_duration)?.vulkan_time(),
        };

        Ok(config)
    }


    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError> {

        if let Some(v) = toml.get("image_count") {
            self.image_count = v.as_integer().ok_or(ConfigError::ParseError)? as u32;
        }

        if let Some(v) = toml.get("framebuffer_layers") {
            self.framebuffer_layers = v.as_integer().ok_or(ConfigError::ParseError)? as u32;
        }

        if let Some(v) = toml.get("prefer_surface_format") {
            self.prefer_surface_format = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
        }

        if let Some(v) = toml.get("prefer_surface_color_space") {
            self.prefer_surface_color_space = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
        }

        if let Some(v) = toml.get("present_mode_primary") {
            self.present_mode_primary = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
        }

        if let Some(v) = toml.get("present_mode_secondary") {
            self.present_mode_secondary = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
        }

        if let Some(v) = toml.get("acquire_image_time_out") {
            self.acquire_image_time_out = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
        }

        if let Some(v) = toml.get("acquire_image_duration") {
            self.acquire_image_duration = v.as_integer().ok_or(ConfigError::ParseError)? as u64;
        }

        Ok(())
    }
}

fn vk_raw2colorspace(raw: &String) -> Result<vk::ColorSpaceKHR, ConfigError> {

    let color_space = match raw.as_str() {
        | "SrgbNonlinear" => vk::ColorSpaceKHR::SRGB_NONLINEAR,
        | _ => return Err(ConfigError::Mapping(MappingError::ColorspaceMappingError)),
    };

    Ok(color_space)
}

fn vk_raw2presentmode(raw: &String) -> Result<vk::PresentModeKHR, ConfigError> {

    let present_mode = match raw.as_str() {
        | "Immediate"   => vk::PresentModeKHR::IMMEDIATE,
        | "Mailbox"     => vk::PresentModeKHR::MAILBOX,
        | "Fifo"        => vk::PresentModeKHR::FIFO,
        | "FifoRelaxed" => vk::PresentModeKHR::FIFO_RELAXED,
        | _ => return Err(ConfigError::Mapping(MappingError::SwapchainPresentModeError)),
    };

    Ok(present_mode)
}

fn vk_raw2acquire_image_time(time_out: &String, duration: u64) -> Result<TimePeriod, ConfigError> {

    let time = match time_out.as_str() {
        | "Infinte"   => TimePeriod::Infinte,
        | "Immediate" => TimePeriod::Immediate,
        | "Timing"    => TimePeriod::Time(Duration::from_millis(duration)),
        | _ => return Err(ConfigError::Mapping(MappingError::SwapchainImageTimeAcqurieError)),
    };

    Ok(time)
}
