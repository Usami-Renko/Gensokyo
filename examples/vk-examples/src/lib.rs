
use lazy_static::lazy_static;

use ash::vk;
use nalgebra::Matrix4;

lazy_static! {

    pub static ref Y_CORRECTION: Matrix4<f32> = Matrix4::new(
        1.0,  0.0, 0.0, 0.0,
        0.0, -1.0, 0.0, 0.0,
        0.0,  0.0, 0.5, 0.5,
        0.0,  0.0, 0.0, 1.0,
    );
}

pub const DEFAULT_CLEAR_COLOR: vk::ClearValue = vk::ClearValue {
    color: vk::ClearColorValue {
        float32: [0.025, 0.025, 0.025, 1.0]
    }
};
