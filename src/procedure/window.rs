
use winit;
use winit::{ VirtualKeyCode, Event, WindowEvent };

use structures::Dimension2D;
use constant::window;

use procedure::workflow::ProgramProc;

pub struct ProgramBuilder<T> {

    window_size:  Dimension2D,
    window_title: String,

    procedure: T,
}

impl <T> ProgramBuilder<T> where T: ProgramProc {

    pub fn new(procedure: T) -> ProgramBuilder<T> {
        ProgramBuilder {
            window_size:  window::WINDOW_SIZE,
            window_title: window::WINDOW_TITLE.to_owned(),

            procedure,
        }
    }

    pub fn title(mut self, title: &str) -> ProgramBuilder<T> {
        self.window_title = title.to_owned();
        self
    }

    pub fn size(mut self, window_width: u32, window_height: u32) -> ProgramBuilder<T> {
        self.window_size = Dimension2D {
            width:  window_width,
            height: window_height,
        };

        self
    }

    pub fn build(self) -> Result<ProgramEnv<T>, winit::CreationError> {

        let event_loop = winit::EventsLoop::new();
        let window = winit::WindowBuilder::new()
            .with_title(self.window_title)
            .with_dimensions((self.window_size.width, self.window_size.height).into())
            .build(&event_loop)?;

        let program_env = ProgramEnv {
            window_size: self.window_size,

            event_loop,
            window,
            procedure: self.procedure,
        };

        Ok(program_env)
    }
}

pub struct ProgramEnv<T: ProgramProc> {

    window_size: Dimension2D,

    event_loop: winit::EventsLoop,
    window: winit::Window,
    procedure: T,
}

impl<T> ProgramEnv<T> where T: ProgramProc {

    pub fn launch(&mut self) {

        // TODO: Refactor the following two lines
        use core::physical::PhysicalRequirement;
        let requirement = PhysicalRequirement::init();

        let _core = self.initialize_core(requirement);
        self.main_loop();
    }

    fn main_loop(&mut self) {

        let mut is_running       = true;
        let mut is_first_resized = true;

        'mainloop: loop {
            self.event_loop.poll_events(|event| {
                match event {
                    // handling keyboard event
                    | Event::WindowEvent { event, .. } => match event {
                        | WindowEvent::KeyboardInput { input, .. } => {
                            if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                                is_running = false;
                            }
                        }
                        | WindowEvent::Resized(_) => {

                            if is_first_resized {
                                is_first_resized = false;
                            } else {
                                // TODO: Implement resized handling.
                                unimplemented!("Resized is not implemented yet")
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

            if is_running == false {
                break 'mainloop
            }
        }
    }


}
