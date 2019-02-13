
use winit;
use smallvec::SmallVec;

use crate::config::input::SIMULTANEOUS_KEY_COUNT;

pub(crate) struct KeyHeap {

    keys: SmallVec<[winit::VirtualKeyCode; SIMULTANEOUS_KEY_COUNT]>,
}

impl KeyHeap {

    pub fn new() -> KeyHeap {
        KeyHeap {
            keys: SmallVec::new(),
        }
    }

    pub fn key_press(&mut self, code: winit::VirtualKeyCode) {

        // if input key has been existed, just ignore it.
        if self.keys.iter().any(|&key_code| key_code == code) {
            return
        }

        // and the key pool has been full, just ignore the input key.
        if self.keys.len() < SIMULTANEOUS_KEY_COUNT {
            self.keys.push(code);
        }
    }

    pub fn key_release(&mut self, code: winit::VirtualKeyCode) {

        if let Some(index) = self.keys.iter().position(|&key_code| key_code == code) {
            self.keys.swap_remove(index);
        }
    }

    // TODO: implement is_action_just_pressed, is_action_just_released, and is_action_pressed.
    pub fn is_key_pressed(&self, code: winit::VirtualKeyCode) -> bool {

        self.keys.iter().any(|&key_code| key_code == code)
    }
}
