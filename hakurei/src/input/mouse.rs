
#[derive(Default)]
pub(crate) struct MouseSensor {

    motion: CursorMotion,
}

impl MouseSensor {

    pub fn new() -> MouseSensor {
        MouseSensor::default()
    }

    pub fn record_motion(&mut self, delta_x: f64, delta_y: f64) {
        self.motion.delta_x = delta_x as f32;
        self.motion.delta_y = delta_y as f32;
    }

    pub fn get_cursor_motion(&self) -> CursorMotion {
        self.motion
    }
}

#[derive(Default, Copy, Clone)]
pub struct CursorMotion {
    pub delta_x: f32,
    pub delta_y: f32,
}

impl CursorMotion {

    pub fn scale(&self, factor: f32) -> CursorMotion {
        CursorMotion {
            delta_x: self.delta_x * factor,
            delta_y: self.delta_y * factor,
        }
    }
}
