
#[macro_use] extern crate lazy_static;

use nalgebra::Matrix4;

lazy_static! {

    pub static ref Y_CORRECTION: Matrix4<f32> = Matrix4::new(
        1.0,  0.0, 0.0, 0.0,
        0.0, -1.0, 0.0, 0.0,
        0.0,  0.0, 0.5, 0.5,
        0.0,  0.0, 0.0, 1.0,
    );
}

