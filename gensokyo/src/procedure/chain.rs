
use crate::config::resources::ResourceConfig;

use crate::procedure::env::{ ProgramEnv, VulkanEnv };
use crate::procedure::loader::AssetsLoader;
use crate::procedure::error::ProcedureError;

use gsvk::core::device::{ GsDevice, DeviceQueueIdentifier };
use gsvk::core::swapchain::{ GsChain, SwapchainConfig };
use gsvk::core::swapchain::error::{ SwapchainError, SwapchainRuntimeError };
use gsvk::sync::{ GsSemaphore, GsFence, SyncError };
use gsvk::types::vkuint;

use crate::utils::time::TimePeriod;

pub(super) struct ChainResource {

    // window instance.
    window: winit::Window,

    // swapchain.
    swapchain: GsChain,
    frame_in_flights: usize,
    current_frame: usize,

    // sync.
    image_awaits : Vec<GsSemaphore>,
    sync_fences  : Vec<GsFence>,
}

impl ChainResource {

    pub fn new(env: &ProgramEnv, window: winit::Window) -> Result<ChainResource, ProcedureError> {

        let swapchain = env.vulkan_env.new_chain(&env.config.core.swapchain, None, &window)?;
        let frame_in_flights = env.config.core.swapchain.image_count as usize;

        let (image_awaits, sync_fences) = create_syncs(&env.vulkan_env.device, frame_in_flights)?;

        let chain = ChainResource {
            window, swapchain, frame_in_flights, image_awaits, sync_fences,
            current_frame: 0,
        };

        Ok(chain)
    }

    pub fn assets_loader(&self, env: &VulkanEnv, config: &ResourceConfig) -> AssetsLoader {

        AssetsLoader::new(&env, config, &self.swapchain)
    }

    pub fn acquire_next_image(&self) -> Result<AcquireImageInfo, ProcedureError> {

        let fence_to_wait = &self.sync_fences[self.current_frame];
        fence_to_wait.wait(TimePeriod::Infinte.vulkan_time())?;

        let image_to_acquire = &self.image_awaits[self.current_frame];

        let image_result = self.swapchain
            .next_image(Some(image_to_acquire), None);
        let acquire_image_index = match image_result {
            | Ok(result) => result,
            | Err(e) => match e {
                | SwapchainRuntimeError::SurfaceOutOfDate
                | SwapchainRuntimeError::SubOptimal => {
                    return Err(ProcedureError::SwapchainRecreate)
                },
                | _ => return Err(ProcedureError::Swapchain(SwapchainError::Runtime(e))),
            }
        };

        fence_to_wait.reset()?;

        let result = AcquireImageInfo {
            device_ready: fence_to_wait,
            image_acquire_finished: image_to_acquire,
            acquire_image_index,
        };

        Ok(result)
    }

    pub fn present_image(&self, device: &GsDevice, present_available: &GsSemaphore, image_index: vkuint) -> Result<(), ProcedureError> {

        // FIXME: Use present queue will cause crash. Image ownership transfer is necessary,
        // see https://github.com/KhronosGroup/Vulkan-Docs/wiki/Synchronization-Examples.
        // or see https://software.intel.com/en-us/articles/api-without-secrets-introduction-to-vulkan-part-3#inpage-nav-6-3
        let present_result = self.swapchain.present(device,
            &[present_available], image_index,
            DeviceQueueIdentifier::Graphics
        );

        if let Err(e) = present_result {
            match e {
                | SwapchainRuntimeError::SurfaceOutOfDate
                | SwapchainRuntimeError::SubOptimal => {
                    return Err(ProcedureError::SwapchainRecreate)
                },
                | _ => return Err(ProcedureError::Swapchain(SwapchainError::Runtime(e))),
            }
        }

        Ok(())
    }

    pub fn next_frame(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.frame_in_flights;
    }

    pub fn reload(&mut self, env: &VulkanEnv, config: &SwapchainConfig) -> Result<(), ProcedureError> {

        let new_chain = env.new_chain(config, Some(&self.swapchain), &self.window)?;
        self.destroy(&env.device);

        self.swapchain = new_chain;
        self.recreate_syncs(&env.device)?;

        Ok(())
    }

    pub fn destroy(&self, device: &GsDevice) {

        self.swapchain.destroy(device);
        self.image_awaits.iter()
            .for_each(|i| i.destroy());
        self.sync_fences.iter()
            .for_each(|f| f.destroy());
    }

    fn recreate_syncs(&mut self, device: &GsDevice) -> Result<(), ProcedureError> {

        self.sync_fences.clear();
        self.image_awaits.clear();

        let (image_awaits, sync_fences) = create_syncs(device, self.frame_in_flights)?;
        self.image_awaits = image_awaits;
        self.sync_fences = sync_fences;

        Ok(())
    }
}

fn create_syncs(device: &GsDevice, frame_in_flights: usize) -> Result<(Vec<GsSemaphore>, Vec<GsFence>), SyncError> {

    let mut image_awaits = vec![];
    let mut sync_fences = vec![];

    for _ in 0..frame_in_flights {
        let image_await = GsSemaphore::setup(device)?;
        let sync_fence = GsFence::setup(device, true)?;

        image_awaits.push(image_await);
        sync_fences.push(sync_fence);
    }

    Ok((image_awaits, sync_fences))
}

pub struct AcquireImageInfo<'sync> {

    pub device_ready: &'sync GsFence,
    pub image_acquire_finished: &'sync GsSemaphore,

    pub acquire_image_index: vkuint,
}