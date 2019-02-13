
use crate::config::resources::ResourceConfig;

use crate::procedure::context::{ ProgramContext, VulkanContext };
use crate::initialize::initializer::AssetInitializer;

use gsvk::core::GsDevice;
use gsvk::core::device::DeviceQueueIdentifier;
use gsvk::core::swapchain::{ GsChain, SwapchainConfig };
use gsvk::sync::{ GsSemaphore, GsFence };
use gsvk::types::vkuint;

use crate::utils::time::TimePeriod;
use crate::error::GsResult;

pub(super) struct ChainResource {

    // window instance.
    window: winit::Window,

    // swapchain.
    swapchain: GsChain,
    frame_in_flights: usize,
    current_frame: usize,

    // sync.
    image_awaits: Vec<GsSemaphore>,
    sync_fences : Vec<GsFence>,
}

impl ChainResource {

    pub fn new(context: &ProgramContext, window: winit::Window) -> GsResult<ChainResource> {

        let swapchain = context.vulkan_context.new_chain(&context.config.core.swapchain, None, &window)?;
        let frame_in_flights = context.config.core.swapchain.image_count as usize;

        let (image_awaits, sync_fences) = create_syncs(&context.vulkan_context.device, frame_in_flights)?;

        let chain = ChainResource {
            window, swapchain, frame_in_flights, image_awaits, sync_fences,
            current_frame: 0,
        };

        Ok(chain)
    }

    pub fn assets_loader(&self, vulkan: &VulkanContext, config: &ResourceConfig) -> AssetInitializer {

        AssetInitializer::create(&vulkan.device, &self.swapchain, config)
    }

    pub fn acquire_next_image(&self) -> GsResult<AcquireImageInfo> {

        let fence_to_wait = &self.sync_fences[self.current_frame];
        fence_to_wait.wait(TimePeriod::Infinite.vulkan_time())?;

        let image_to_acquire = &self.image_awaits[self.current_frame];

        let acquire_image_index = self.swapchain
            .next_image(Some(image_to_acquire), None)?;

        fence_to_wait.reset()?;

        let result = AcquireImageInfo {
            device_ready: fence_to_wait,
            image_acquire_finished: image_to_acquire,
            acquire_image_index,
        };

        Ok(result)
    }

    pub fn present_image(&self, device: &GsDevice, present_available: &GsSemaphore, image_index: vkuint) -> GsResult<()> {

        // FIXME: Use present queue will cause crash. Image ownership transfer is necessary,
        // see https://github.com/KhronosGroup/Vulkan-Docs/wiki/Synchronization-Examples.
        // or see https://software.intel.com/en-us/articles/api-without-secrets-introduction-to-vulkan-part-3#inpage-nav-6-3
        self.swapchain.present(device,
            &[present_available], image_index,
            DeviceQueueIdentifier::Graphics
        )?;

        Ok(())
    }

    pub fn next_frame(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.frame_in_flights;
    }

    pub fn reload(&mut self, vulkan: &VulkanContext, config: &SwapchainConfig) -> GsResult<()> {

        let new_chain = vulkan.new_chain(config, Some(&self.swapchain), &self.window)?;
        self.discard(&vulkan.device);

        self.swapchain = new_chain;
        self.recreate_syncs(&vulkan.device)?;

        Ok(())
    }

    pub fn discard(&self, device: &GsDevice) {

        self.swapchain.discard(device);
        // image_awaits and sync_fences will be drop in its drop func,
    }

    fn recreate_syncs(&mut self, device: &GsDevice) -> GsResult<()> {

        self.sync_fences.clear();
        self.image_awaits.clear();

        let (image_awaits, sync_fences) = create_syncs(device, self.frame_in_flights)?;
        self.image_awaits = image_awaits;
        self.sync_fences = sync_fences;

        Ok(())
    }
}

fn create_syncs(device: &GsDevice, frame_in_flights: usize) -> GsResult<(Vec<GsSemaphore>, Vec<GsFence>)> {

    let mut image_awaits = vec![];
    let mut sync_fences = vec![];

    for _ in 0..frame_in_flights {
        let image_await = GsSemaphore::create(device)?;
        let sync_fence = GsFence::create(device, true)?;

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
