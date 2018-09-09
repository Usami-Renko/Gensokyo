
use winit;
use winit::{ VirtualKeyCode, Event, WindowEvent };

use utility::dimension::Dimension2D;
use config::engine::EngineConfig;

use procedure::workflow::{ CoreInfrastructure, HaResources, ProgramProc };
use procedure::error::RuntimeError;
use procedure::error::ProcedureError;

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

    pub fn new(config: EngineConfig, procedure: T) -> ProgramEnv<T> {

        let window_info = WindowInfo {
            window_size:  config.window.dimension,
            window_title: config.window.title.to_owned(),
        };
        let event_loop = winit::EventsLoop::new();
        let frame_in_flights = config.swapchain.image_count as usize;

        ProgramEnv { config, event_loop, window_info, procedure, frame_in_flights, }
    }

    pub fn launch(&mut self) -> Result<(), RuntimeError> {

        // TODO: Refactor the following two lines
        use core::physical::PhysicalRequirement;
        use config::core::DEVICE_EXTENSION;
        let requirement = PhysicalRequirement::init(&self.config)
            .require_queue_extensions(DEVICE_EXTENSION.to_vec());

        let window = self.window_info.build(&self.event_loop)
            .map_err(|e| RuntimeError::Window(e))?;
        let mut core = self.initialize_core(&window, requirement)?;
        let mut resources = self.load_resources(&core)?;


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


        self.wait_idle(&core.device)?;

        self.procedure.cleanup(&core.device);
        resources.cleanup(&core.device);
        core.cleanup();

        Ok(())
    }

    fn main_loop(&mut self, core: &mut CoreInfrastructure, resources: &mut HaResources) -> Result<(), ProcedureError> {

        let mut is_running        = true;
        let mut is_first_resized  = true;
        let mut is_resized_tiggle = false;
        let mut current_fame = 0_usize;

        'mainloop: loop {
            self.event_loop.poll_events(|event| {
                match event {
                    // handling keyboard event
                    | Event::WindowEvent { event, .. } => match event {
                        | WindowEvent::KeyboardInput { input, .. } => {
                            if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                                is_running = false;
                            }
                        },
                        | WindowEvent::Resized(_) => {

                            if is_first_resized {
                                is_first_resized = false;
                            } else {
                                is_running        = false;
                                is_resized_tiggle = true;
                            }
                        },
                        | WindowEvent::CloseRequested => {
                            is_running = false;
                        },
                        | _ => (),
                    },
                    | _ => (),
                }
            });

            match self.draw_frame(current_fame, core, resources) {
                | Ok(_) => (),
                | Err(error) => match error {
                    | ProcedureError::SwapchainRecreate => {
                        is_running        = false;
                        is_resized_tiggle = true;
                    },
                    | _ => return Err(error)
                }
            }

            if is_running == false {
                if is_resized_tiggle {
                    return Err(ProcedureError::SwapchainRecreate)
                } else {
                    break 'mainloop
                }
            }

            current_fame = (current_fame + 1) % self.frame_in_flights;
        }

        Ok(())
    }
}
