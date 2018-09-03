
use winit;
use winit::{ VirtualKeyCode, Event, WindowEvent };

use utility::dimension::Dimension2D;
use constant::window;
use constant::sync::SYNCHRONOUT_FRAME;

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

pub struct ProgramBuilder<T> {

    window_info: WindowInfo,
    procedure: T,
}

impl <T> ProgramBuilder<T> where T: ProgramProc {

    pub fn new(procedure: T) -> ProgramBuilder<T> {
        ProgramBuilder {
            window_info: WindowInfo {
                window_size:  window::WINDOW_SIZE,
                window_title: window::WINDOW_TITLE.to_owned(),
            },

            procedure,
        }
    }

    pub fn title(mut self, title: &str) -> ProgramBuilder<T> {
        self.window_info.window_title = title.to_owned();
        self
    }

    pub fn size(mut self, window_width: u32, window_height: u32) -> ProgramBuilder<T> {
        self.window_info.window_size = Dimension2D {
            width:  window_width,
            height: window_height,
        };

        self
    }

    pub fn build(self) -> ProgramEnv<T> {
        ProgramEnv {
            event_loop  : winit::EventsLoop::new(),
            window_info : self.window_info,
            procedure   : self.procedure,
        }
    }
}

pub struct ProgramEnv<T: ProgramProc> {

    event_loop: winit::EventsLoop,
    window_info: WindowInfo,

    pub(super) procedure: T,
}

impl<T> ProgramEnv<T> where T: ProgramProc {

    pub fn launch(&mut self) -> Result<(), RuntimeError> {

        // TODO: Refactor the following two lines
        use core::physical::PhysicalRequirement;
        use constant::core::DEVICE_EXTENSION;
        let requirement = PhysicalRequirement::init()
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
                        resources.cleanup(&core.device);
                        resources.clear();
                        self.procedure.clean_resources(&core.device)?;
                        resources = self.reload_resources(&core)?;

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

            current_fame = (current_fame + 1) % SYNCHRONOUT_FRAME;
        }

        Ok(())
    }
}
