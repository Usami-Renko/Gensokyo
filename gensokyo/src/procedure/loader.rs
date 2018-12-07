
use gsvk::core::device::GsDevice;
use gsvk::core::physical::GsPhyDevice;
use gsvk::core::swapchain::GsChain;

use gsvk::types::vkDim2D;

use crate::config::resources::ResourceConfig;

use crate::toolkit::{ AllocatorKit, PipelineKit, CommandKit, SyncKit };

use crate::procedure::env::VulkanEnv;
use crate::procedure::error::ProcedureError;

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

    pub fn assets<A, F>(&self, func: F) -> Result<A, ProcedureError>
        where F: FnOnce(AllocatorKit) -> Result<A, ProcedureError> {

        let kit = AllocatorKit::init(&self.physical, &self.device, &self.swapchain, self.config.clone());

        func(kit)
    }

    pub fn pipelines<P, F>(&self, func: F) -> Result<P, ProcedureError>
        where F: FnOnce(PipelineKit) -> Result<P, ProcedureError> {

        let kit = PipelineKit::init(&self.device, &self.swapchain);

        func(kit)
    }

    pub fn syncs<R, F>(&self, func: F) -> Result<R, ProcedureError>
        where F: FnOnce(SyncKit) -> Result<R, ProcedureError> {

        let kit = SyncKit::init(&self.device);

        func(kit)
    }

    pub fn commands<C, F>(&self, func: F) -> Result<C, ProcedureError>
        where F: FnOnce(CommandKit) -> Result<C, ProcedureError> {

        let kit = CommandKit::init(&self.device);

        func(kit)
    }

    pub fn screen_dimension(&self) -> vkDim2D {
        self.swapchain.extent()
    }
}
