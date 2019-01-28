
use gsvk::core::device::GsDevice;
use gsvk::core::physical::GsPhyDevice;
use gsvk::core::swapchain::GsChain;

use gsvk::types::vkDim2D;

use crate::toolkit::{ AllocatorKit, PipelineKit, CommandKit, SyncKit };
use crate::procedure::env::VulkanEnv;
use crate::config::resources::ResourceConfig;

pub struct AssetsLoader {

    config: ResourceConfig,

    device    : GsDevice,
    physical  : GsPhyDevice,
    swapchain : GsChain,
}

impl AssetsLoader {

    pub(super) fn new(env: &VulkanEnv, config: &ResourceConfig, swapchain: &GsChain) -> AssetsLoader {

        AssetsLoader {
            config: config.clone(),
            swapchain: swapchain.clone(),
            device   : env.device.clone(),
            physical : env.physical.clone(),
        }
    }

    pub fn assets<R, F>(&self, func: F) -> R
        where
            F: FnOnce(AllocatorKit) -> R {

        let kit = AllocatorKit::init(&self.physical, &self.device, self.config.clone());

        func(kit)
    }

    pub fn pipelines<R, F>(&self, func: F) -> R
        where
            F: FnOnce(PipelineKit) -> R {

        let kit = PipelineKit::init(&self.device, &self.swapchain);

        func(kit)
    }

    pub fn syncs<R, F>(&self, func: F) -> R
        where
            F: FnOnce(SyncKit) -> R {

        let kit = SyncKit::init(&self.device);

        func(kit)
    }

    pub fn commands<R, F>(&self, func: F) -> R
        where
            F: FnOnce(CommandKit) -> R {

        let kit = CommandKit::init(&self.device);

        func(kit)
    }

    pub fn screen_dimension(&self) -> vkDim2D {
        self.swapchain.dimension()
    }
}
