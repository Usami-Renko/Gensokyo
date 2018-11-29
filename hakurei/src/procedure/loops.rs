
use winit;

use procedure::env::ProgramEnv;
use procedure::chain::ChainResource;
use procedure::workflow::GraphicsRoutine;
use procedure::error::{ RuntimeError, ProcedureError };

use utils::fps::HaFpsTimer;

use gsvk::core::device::HaDevice;

use input::{ ActionNerve, SceneReaction };

pub struct RoutineFlow<Routine> where Routine: GraphicsRoutine {

    chain: ChainResource,
    routine: Routine,
}

impl<Routine> RoutineFlow<Routine> where Routine: GraphicsRoutine {

    pub(super) fn new(routine: Routine, chain: ChainResource) -> RoutineFlow<Routine> {

        RoutineFlow {
            routine, chain,
        }
    }

    pub fn launch(&mut self, env: ProgramEnv) -> Result<(), RuntimeError> {

        let (mut window_env, vulkan_env, config) = env.split();
        let device = &vulkan_env.device;
        self.routine.ready(device)?;

        'outer_loop: loop {

            match self.main_loop(device, window_env.borrow_mut_loops()) {
                | Ok(_) => break,
                | Err(error) => match error {
                    | ProcedureError::SwapchainRecreate => {

                        self.wait_device_idle(device)?;
                        self.routine.clean_resources(device)?;
                        self.chain.reload(&vulkan_env, &config.core.swapchain)?;

                        let asset_loader = self.chain.assets_loader(&vulkan_env, &config.resources);
                        self.routine.reload_res(asset_loader)?;

                        continue
                    },
                    | _ => return Err(RuntimeError::Procedure(error))
                }
            }
        }

        self.routine.closure(device)?;
        self.wait_device_idle(device)?;
        self.routine.clean_routine(device);
        self.chain.cleanup(device);

        vulkan_env.cleanup();

        Ok(())
    }

    fn main_loop(&mut self, device: &HaDevice, event_loop: &mut winit::EventsLoop) -> Result<(), ProcedureError> {

        let mut actioner = ActionNerve::new();
        let mut fps_timer = HaFpsTimer::new();

        'innerloop: loop {

            let delta_time = fps_timer.delta_time();

            event_loop.poll_events(|event| {
                actioner.record_event(event);
            });

            let app_action = self.routine.react_input(&actioner, delta_time);
            actioner.cover_reaction(app_action);

            match self.draw_frame(device, delta_time) {
                | Ok(_) => (),
                | Err(error) => match error {
                    | ProcedureError::SwapchainRecreate => {
                        actioner.force_reaction(SceneReaction::SwapchainRecreate)
                    },
                    | _ => return Err(error)
                }
            }

            let reaction = actioner.get_reaction();
            match reaction {
                | SceneReaction::Rendering => {},
                | SceneReaction::SwapchainRecreate => {
                    return Err(ProcedureError::SwapchainRecreate)
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

    fn draw_frame(&mut self, device: &HaDevice, delta_time: f32) -> Result<(), ProcedureError> {

        let acquire_result = self.chain.acquire_next_image()?;

        let image_ready_to_present = self.routine.draw(&device,
            acquire_result.device_ready, acquire_result.image_acquire_finished,
            acquire_result.acquire_image_index as usize, delta_time
        )?;

        self.chain.present_image(device, image_ready_to_present, acquire_result.acquire_image_index)
    }

    fn wait_device_idle(&self, device: &HaDevice) -> Result<(), ProcedureError> {

        device.wait_idle()
            .map_err(|e| ProcedureError::LogicalDevice(e))
    }
}
