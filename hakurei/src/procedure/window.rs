
use winit;

use config::engine::EngineConfig;
use config::env::{ HaEnv, EnvWindow };

use procedure::workflow::{ CoreInfrastructure, HaResources, ProgramProc };
use procedure::error::{ RuntimeError, ProcedureError };

use utility::dimension::Dimension2D;
use utility::fps::HaFpsTimer;

use input::{ ActionNerve, SceneReaction };

use std::path::PathBuf;

struct WindowInfo {
    window_size : Dimension2D,
    window_title: String,
}

impl WindowInfo {
    fn build(&self, event_loop: &winit::EventsLoop) -> Result<winit::Window, winit::CreationError> {
        winit::WindowBuilder::new()
            .with_title(self.window_title.clone())
            .with_dimensions((self.window_size.width, self.window_size.height).into())
            .build(event_loop)
    }
}

pub struct ProgramEnv<T: ProgramProc> {

    event_loop: winit::EventsLoop,
    window_info: WindowInfo,
    frame_in_flights: usize,

    pub(super) config: EngineConfig,
    pub(super) procedure: T,
}

impl<T> ProgramEnv<T> where T: ProgramProc {

    pub fn new(manifest: Option<PathBuf>, procedure: T) -> Result<ProgramEnv<T>, RuntimeError> {

        let config = EngineConfig::init(manifest)?;

        let window_info = WindowInfo {
            window_size : config.window.dimension,
            window_title: config.window.title.to_owned(),
        };
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
                match event {
                    | winit::Event::WindowEvent { event, .. } => {
                        actioner.record_event(&event);
                    },
                    | _ => (),
                }
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
        }

        Ok(())
    }

    fn get_env(&self) -> HaEnv {


        HaEnv {
            window: EnvWindow {
                title: self.config.window.title.clone(),
                dimension: Dimension2D {

                }
            }
        }
    }
}
