
use winit::{ WindowEvent, ElementState, VirtualKeyCode };

use procedure::input::key::KeyHeap;
use procedure::input::keycode::HaKeycode;

pub struct ActionNerve {

    is_active: bool,
    react: SceneReaction,

    key: KeyHeap,
}

impl ActionNerve {

    pub fn new() -> ActionNerve {
        ActionNerve {
            is_active: false,
            react: SceneReaction::Rendering,
            key: KeyHeap::new(),
        }
    }

    pub(crate) fn record_event(&mut self, event: &WindowEvent) {

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

        self.react = SceneReaction::Rendering;
    }

    pub fn is_key_pressed(&self, key_code: HaKeycode) -> bool {
        self.key.is_key_pressed(VirtualKeyCode::from(key_code))
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
