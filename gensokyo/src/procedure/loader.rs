
use gsvk::core::device::GsDevice;
use gsvk::core::physical::GsPhyDevice;
use gsvk::core::swapchain::GsSwapchain;

use config::resources::ResourceConfig;

use toolkit::{ AllocatorKit, PipelineKit, CommandKit };

use procedure::env::VulkanEnv;
use procedure::error::ProcedureError;

pub struct AssetsLoader<'a> {

    config: ResourceConfig,

    device    : GsDevice,
    physical  : GsPhyDevice,
    swapchain : &'a GsSwapchain,
}

impl<'a> AssetsLoader<'a> {

    pub(super) fn new(env: &VulkanEnv, config: &ResourceConfig, swapchain: &'a GsSwapchain) -> AssetsLoader<'a> {

        AssetsLoader {
            config: config.clone(),
            swapchain,
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
        where F: FnOnce(PipelineKit, &GsSwapchain) -> Result<P, ProcedureError> {

        let kit = PipelineKit::init(&self.device);

        func(kit, &self.swapchain)
    }

    pub fn subresources<R, F>(&self, func: F) -> Result<R, ProcedureError>
        where F: FnOnce(&GsDevice) -> Result<R, ProcedureError> {

        func(&self.device)
    }

    pub fn commands<C, F>(&self, func: F) -> Result<C, ProcedureError>
        where F: FnOnce(CommandKit) -> Result<C, ProcedureError> {

        let kit = CommandKit::init(&self.device);

        func(kit)
    }
}
