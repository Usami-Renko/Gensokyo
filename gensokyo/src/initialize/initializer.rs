
use gsvk::core::GsDevice;
use gsvk::core::swapchain::GsChain;

use gsvk::types::vkDim2D;

use crate::config::resources::ResourceConfig;

pub struct AssetInitializer {

    pub(super) device   : GsDevice,
    pub(super) swapchain: GsChain,

    pub(super) config: ResourceConfig,
}

impl AssetInitializer {

    pub(crate) fn create(device: &GsDevice, chain: &GsChain, config: &ResourceConfig) -> AssetInitializer {

        AssetInitializer {
            device: device.clone(),
            swapchain: chain.clone(),
            config: config.clone(),
        }
    }

    pub fn screen_dimension(&self) -> vkDim2D {
        self.swapchain.dimension()
    }
}
