
use crate::procedure::context::{ ProgramContext, VulkanContext, WindowContext };
use crate::procedure::chain::ChainResource;
use crate::procedure::workflow::GraphicsRoutine;

use crate::config::engine::EngineConfig;
use crate::input::{ ActionNerve, SceneReaction };
use crate::error::{ GsResult, GsError, GsErrorKind };
use crate::utils::fps::GsFpsTimer;

use gsvk::core::GsDevice;
use gsvk::core::swapchain::SwapchainSyncError;
use gsvk::error::VkErrorKind;

pub struct RoutineFlow<Routine>
    where
        Routine: GraphicsRoutine {

    chain: ChainResource,
    routine: Routine,
}

impl<Routine> RoutineFlow<Routine>
    where
        Routine: GraphicsRoutine {

    pub(super) fn new(routine: Routine, chain: ChainResource) -> RoutineFlow<Routine> {

        RoutineFlow { routine, chain }
    }

    pub fn launch(mut self, context: ProgramContext) -> GsResult<()> {

        let (window_context, vulkan_context, config) = context.take();
        let device = &vulkan_context.device;

        self.routine.ready(device)?;

        self.main_loop(window_context, &vulkan_context, config)?;

        self.routine.closure(device)?;
        self.wait_device_idle(device)?;
        self.chain.discard(device);

        // free the program specific resource.
        drop(self);
        // and then free vulkan environment resource.
        vulkan_context.discard();

        Ok(())
    }

    fn main_loop(&mut self, window_context: WindowContext, vulkan_context: &VulkanContext, config: EngineConfig) -> GsResult<()> {

        let device = &vulkan_context.device;
        let mut window = window_context;

        let mut actioner = ActionNerve::new();
        let mut fps_timer = GsFpsTimer::new();

        'innerloop: loop {

            let delta_time = fps_timer.delta_time();

            window.event_loop.poll_events(|event| {
                actioner.record_event(event);
            });

            let app_action = self.routine.react_input(&actioner, delta_time);
            actioner.cover_reaction(app_action);

            match self.draw_frame(device, delta_time) {
                | Ok(_) => (),
                | Err(error) => {
                    if error.is_swapchain_recreate() {
                        actioner.force_reaction(SceneReaction::SwapchainRecreate);
                    } else {
                        return Err(error)
                    }
                }
            };

            match actioner.get_reaction() {
                | SceneReaction::Rendering => {},
                | SceneReaction::SwapchainRecreate => {

                    self.wait_device_idle(device)?;

                    self.chain.reload(&vulkan_context, &config.core.swapchain)?;

                    let asset_loader = self.chain.assets_loader(&vulkan_context, &config.resources);
                    self.routine.reload_res(asset_loader)?;
                },
                | SceneReaction::Terminate => {
                    break 'innerloop
                },
            }

            self.chain.next_frame();

            fps_timer.tick_frame();
            actioner.reset_frame();
        }

        Ok(())
    }

    fn draw_frame(&mut self, device: &GsDevice, delta_time: f32) -> GsResult<()> {

        let acquire_result = self.chain.acquire_next_image()?;

        let image_ready_to_present = self.routine.draw(&device,
            acquire_result.device_ready, acquire_result.image_acquire_finished,
            acquire_result.acquire_image_index as _, delta_time
        )?;

        self.chain.present_image(device, image_ready_to_present, acquire_result.acquire_image_index)
    }

    fn wait_device_idle(&self, device: &GsDevice) -> GsResult<()> {

        device.logic.wait_idle()?; Ok(())
    }
}


impl GsError {

    fn is_swapchain_recreate(&self) -> bool {

        if let GsErrorKind::Vk(error) = self.kind() {
            if let VkErrorKind::SwapchainSync(swapchain_error) = error.kind() {
                match swapchain_error {
                    | SwapchainSyncError::SurfaceOutDate
                    | SwapchainSyncError::SubOptimal => {
                        return true
                    },
                    | _ => {},
                }
            }
        }

        false
    }
}
