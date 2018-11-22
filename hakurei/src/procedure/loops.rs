
use winit;

use config::engine::EngineConfig;
use config::env::HaEnv;
use config::error::ConfigError;

use procedure::window::WindowInfo;
use procedure::workflow::{ CoreInfrastructure, HaResources, ProgramProc };
use procedure::error::{ RuntimeError, ProcedureError };

use utils::fps::HaFpsTimer;
use vk::utils::types::vkDimension2D;

use input::{ ActionNerve, SceneReaction };

use std::path::PathBuf;

pub struct ProgramEnv<T: ProgramProc> {

    event_loop: winit::EventsLoop,
    window_info: WindowInfo,
    frame_in_flights: usize,

    pub(super) config: EngineConfig,
    pub(super) procedure: T,
}

impl<T> ProgramEnv<T> where T: ProgramProc {

    pub fn new(manifest: Option<PathBuf>, procedure: T) -> Result<ProgramEnv<T>, ConfigError> {

        let config = EngineConfig::init(manifest)?;

        let window_info = WindowInfo::from(config.window.clone());
        let event_loop = winit::EventsLoop::new();
        let frame_in_flights = config.core.swapchain.image_count as usize;

        let program = ProgramEnv { config, event_loop, window_info, procedure, frame_in_flights };
        Ok(program)
    }

    pub fn launch(&mut self) -> Result<(), RuntimeError> {

        let window = self.window_info.build(&self.event_loop)
            .map_err(|e| RuntimeError::Window(e))?;
        let mut core = self.initialize_core(&window, &self.config)?;
        let mut resources = self.load_resources(&core)?;

        self.procedure.ready(&core.device)?;

        'outer_loop: loop {
            match self.main_loop(&mut core, &mut resources) {
                | Ok(_) => break,
                | Err(error) => match error {
                    | ProcedureError::SwapchainRecreate => {
                        self.wait_idle(&core.device)?;
                        self.procedure.clean_resources(&core.device)?;
                        let new_resources = self.reload_resources(&core, &resources)?;
                        resources.cleanup(&core.device);
                        resources.clear();

                        resources = new_resources;

                        // update current window size.
                        // TODO: Handle unwrap().
                        let window_size = window.get_inner_size().unwrap();
                        let dimension = vkDimension2D {
                            width : window_size.width  as u32,
                            height: window_size.height as u32,
                        };
                        self.window_info.reset_size(dimension);

                        continue
                    },
                    | _ => return Err(RuntimeError::Procedure(error))
                }
            }
        }

        self.procedure.closure(&core.device)?;

        self.wait_idle(&core.device)?;

        self.procedure.cleanup(&core.device);
        resources.cleanup(&core.device);
        core.cleanup();

        Ok(())
    }

    fn main_loop(&mut self, core: &mut CoreInfrastructure, resources: &mut HaResources) -> Result<(), ProcedureError> {

        let mut actioner = ActionNerve::new();
        let mut current_fame = 0_usize;
        let mut fps_timer = HaFpsTimer::new();

        'mainloop: loop {

            let delta_time = fps_timer.delta_time();

            self.event_loop.poll_events(|event| {
                actioner.record_event(event);
            });

            let app_action = self.procedure.react_input(&actioner, delta_time);
            actioner.cover_reaction(app_action);

            match self.draw_frame(current_fame, core, resources, delta_time) {
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
                    break 'mainloop
                }
            }

            current_fame = (current_fame + 1) % self.frame_in_flights;
            fps_timer.tick_frame();
            actioner.reset_frame();
        }

        Ok(())
    }

    pub fn gen_env(&self) -> HaEnv {

        self.window_info.gen_env()
    }
}