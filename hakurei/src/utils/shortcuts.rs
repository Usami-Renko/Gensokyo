
use vk::utils::types::{ vkint, vkMemorySize };

use config::error::ConfigError;

pub fn spaces_to_offsets(spaces: &Vec<vkMemorySize>) -> Vec<vkMemorySize> {

    let mut current: vkMemorySize = 0;
    let mut offsets = vec![];

    for &space in spaces.iter() {
        offsets.push(current);
        current += space;
    }

    offsets
}

pub fn vk_to_version(raw_version: &String) -> Result<vkint, ConfigError> {

    let versions = raw_version.split('.').collect::<Vec<_>>();

    if versions.len() == 3 {

        let major = versions[0].parse::<vkint>().map_err(|_| ConfigError::ParseError)?;
        let minor = versions[1].parse::<vkint>().map_err(|_| ConfigError::ParseError)?;
        let patch = versions[2].parse::<vkint>().map_err(|_| ConfigError::ParseError)?;

        use vk::utils::cast::vk_make_version;
        let verion = vk_make_version(major, minor, patch);

        Ok(verion)
    } else {
        return Err(ConfigError::ParseError)
    }
}
