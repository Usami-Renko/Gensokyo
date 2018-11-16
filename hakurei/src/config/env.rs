
use vk::utils::types::vkDimension2D;

pub struct HaEnv {

    pub window: EnvWindow,
}

pub struct EnvWindow {

    pub title: String,
    pub dimension: vkDimension2D,
}
