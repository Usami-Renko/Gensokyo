
use toml;
use ash::vk;

use crate::config::engine::ConfigMirror;
use crate::utils::time::TimePeriod;
use crate::error::{ GsResult, GsError };

use gsvk::core::swapchain::SwapchainConfig;
use gsvk::types::vkuint;

use std::time::Duration;

#[derive(Deserialize)]
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

impl Default for SwapchainConfigMirror {

    fn default() -> SwapchainConfigMirror {
        SwapchainConfigMirror {
            image_count: 2,
            framebuffer_layers: 1,
            prefer_surface_format     : String::from("B8G8R8A8_UNORM"),
            prefer_surface_color_space: String::from("SrgbNonlinear"),
            present_mode_primary  : String::from("Mailbox"),
            present_mode_secondary: String::from("Fifo"),
            acquire_image_time_out: String::from("Infinite"),
            acquire_image_duration: 1000_u64,
        }
    }
}

impl ConfigMirror for SwapchainConfigMirror {
    type ConfigType = SwapchainConfig;

    fn into_config(self) -> GsResult<Self::ConfigType> {

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


    fn parse(&mut self, toml: &toml::Value) -> GsResult<()> {

        if let Some(v) = toml.get("image_count") {
            self.image_count = v.as_integer()
                .ok_or(GsError::config("image_count"))? as u32;
        }

        if let Some(v) = toml.get("framebuffer_layers") {
            self.framebuffer_layers = v.as_integer()
                .ok_or(GsError::config("framebuffer_layers"))? as u32;
        }

        if let Some(v) = toml.get("prefer_surface_format") {
            self.prefer_surface_format = v.as_str()
                .ok_or(GsError::config("prefer_surface_format"))?.to_owned();
        }

        if let Some(v) = toml.get("prefer_surface_color_space") {
            self.prefer_surface_color_space = v.as_str()
                .ok_or(GsError::config("prefer_surface_color_space"))?.to_owned();
        }

        if let Some(v) = toml.get("present_mode_primary") {
            self.present_mode_primary = v.as_str()
                .ok_or(GsError::config("present_mode_primary"))?.to_owned();
        }

        if let Some(v) = toml.get("present_mode_secondary") {
            self.present_mode_secondary = v.as_str()
                .ok_or(GsError::config("present_mode_secondary"))?.to_owned();
        }

        if let Some(v) = toml.get("acquire_image_time_out") {
            self.acquire_image_time_out = v.as_str()
                .ok_or(GsError::config("acquire_image_time_out"))?.to_owned();
        }

        if let Some(v) = toml.get("acquire_image_duration") {
            self.acquire_image_duration = v.as_integer()
                .ok_or(GsError::config("acquire_image_duration"))? as u64;
        }

        Ok(())
    }
}

fn vk_raw2colorspace(raw: &String) -> GsResult<vk::ColorSpaceKHR> {

    let color_space = match raw.as_str() {
        | "SrgbNonlinear" => vk::ColorSpaceKHR::SRGB_NONLINEAR,
        | _ => return Err(GsError::config(raw)),
    };

    Ok(color_space)
}

fn vk_raw2presentmode(raw: &String) -> GsResult<vk::PresentModeKHR> {

    let present_mode = match raw.as_str() {
        | "Immediate"   => vk::PresentModeKHR::IMMEDIATE,
        | "Mailbox"     => vk::PresentModeKHR::MAILBOX,
        | "Fifo"        => vk::PresentModeKHR::FIFO,
        | "FifoRelaxed" => vk::PresentModeKHR::FIFO_RELAXED,
        | _ => return Err(GsError::config(raw)),
    };

    Ok(present_mode)
}

fn vk_raw2acquire_image_time(time_out: &String, duration: u64) -> GsResult<TimePeriod> {

    let time = match time_out.as_str() {
        | "Infinite"   => TimePeriod::Infinite,
        | "Immediate" => TimePeriod::Immediate,
        | "Timing"    => TimePeriod::Time(Duration::from_millis(duration)),
        | _ => return Err(GsError::config(time_out)),
    };

    Ok(time)
}
