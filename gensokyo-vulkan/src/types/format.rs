
use ash::vk;

pub enum VKFormat {

    Undefine,
    Rgba8Unorm,
    D32Sfloat,
}

impl VKFormat {

    pub fn value(&self) -> vk::Format {
        match self {
            | VKFormat::Undefine => vk::Format::UNDEFINED,
            | VKFormat::Rgba8Unorm => vk::Format::R8G8B8A8_UNORM,
            | VKFormat::D32Sfloat => vk::Format::D32_SFLOAT,
        }
    }
}
