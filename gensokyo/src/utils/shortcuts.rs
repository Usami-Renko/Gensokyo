
use gsvk::types::vkuint;

use config::error::ConfigError;

pub fn vk_to_version(raw_version: &String) -> Result<vkuint, ConfigError> {

    let versions: Vec<_> = raw_version.split('.').collect();

    if versions.len() == 3 {

        let major = versions[0].parse::<vkuint>().map_err(|_| ConfigError::ParseError)?;
        let minor = versions[1].parse::<vkuint>().map_err(|_| ConfigError::ParseError)?;
        let patch = versions[2].parse::<vkuint>().map_err(|_| ConfigError::ParseError)?;

        let verion = vk_make_version!(major, minor, patch);

        Ok(verion)
    } else {
        return Err(ConfigError::ParseError)
    }
}
