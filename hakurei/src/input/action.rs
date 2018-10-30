
use winit;
use winit::{ DeviceEvent, WindowEvent, ElementState, VirtualKeyCode };

use input::key::KeyHeap;
use input::mouse::{ MouseSensor, CursorMotion };
use input::keycode::HaKeycode;

pub struct ActionNerve {

    is_active: bool,
    state: SceneState,
    react: SceneReaction,

    key  : KeyHeap,
    mouse: MouseSensor,
}

impl ActionNerve {

    pub fn new() -> ActionNerve {
        ActionNerve {
            is_active: false,
            state: SceneState::new(),
            react: SceneReaction::Rendering,
            key  : KeyHeap::new(),
            mouse: MouseSensor::new(),
        }
    }

    pub(crate) fn record_event(&mut self, event: winit::Event) {

        match event {
            | winit::Event::DeviceEvent { device_id: _, event } => {
                match event {
                    | DeviceEvent::MouseMotion { delta } => {
                        self.state.toggle_mouse_motion();
                        self.mouse.record_motion(delta.0, delta.1);
                    },
                    | _ => (),
                }
            },
            | winit::Event::WindowEvent { window_id: _, event } => {
                match event {
                    | WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(code) = input.virtual_keycode {
                            match input.state {
                                | ElementState::Pressed  => self.key.key_press(code),
                                | ElementState::Released => self.key.key_release(code),
                            }
                        }
                    },
                    | WindowEvent::Resized(_) => {

                        if self.is_active {
                            self.react = SceneReaction::SwapchainRecreate;
                        } else {
                            self.is_active = true;
                        }

                        return
                    },
                    | WindowEvent::CloseRequested => {
                        self.react = SceneReaction::Terminate;
                        return
                    },
                    | _ => (),
                }
            },
            | _ => (),
        }

        self.react = SceneReaction::Rendering;
    }

    // TODO: this function may waste time to get the keycode.
    pub fn is_key_pressed(&self, key_code: HaKeycode) -> bool {
        self.key.is_key_pressed(VirtualKeyCode::from(key_code))
    }

    pub(crate) fn is_key_pressed_raw(&self, key_code: VirtualKeyCode) -> bool {
        self.key.is_key_pressed(key_code)
    }

    pub fn is_mouse_move(&self) -> bool {
        self.state.is_cursor_move
    }

    pub fn mouse_motion(&self) -> CursorMotion {
        self.mouse.get_cursor_motion()
    }

    pub fn get_reaction(&self) -> SceneReaction {
        self.react
    }
    pub fn force_reaction(&mut self, reaction: SceneReaction) {
        self.react = reaction;
    }
    pub fn cover_reaction(&mut self, action: SceneAction) {
        // TODO: implement Pause and Resume function.
        match action {
            | SceneAction::Rendering => {
                // do nothing
            },
            | SceneAction::Terminal  => self.react = SceneReaction::Terminate,
            | SceneAction::Pause => {
                unimplemented!()
            },
            | SceneAction::Resume => {
                unimplemented!()
            },
        }
    }

    pub(crate) fn reset_frame(&mut self) {
        self.state.reset();
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SceneReaction {

    Rendering,
    SwapchainRecreate,
    Terminate,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SceneAction {

    Rendering,
    Pause,
    Resume,
    Terminal,
}

struct SceneState {

    is_cursor_move: bool,
}

impl SceneState {

    fn new() -> SceneState {
        SceneState {
            is_cursor_move: false,
        }
    }

    fn toggle_mouse_motion(&mut self) {
        self.is_cursor_move = true;
    }

    fn reset(&mut self) {
        self.is_cursor_move = false;
    }
}
