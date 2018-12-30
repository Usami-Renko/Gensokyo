
use nalgebra::{ Matrix4, Point3, Vector3, RowVector4, Vector4 };

use crate::input::ActionNerve;

pub trait GsCameraAbstract {

    /// Return the view matrix base on the properties of camera.
    fn view_matrix(&self) -> Matrix4<f32>;
    /// Return the projection matrix base on the properties of camera.
    fn proj_matrix(&self) -> Matrix4<f32>;
    /// Reset the screen dimension to camera, so that the camera can fill the screen aspect.
    fn reset_screen_dimension(&mut self, width: u32, height: u32);
    /// Call the method to tell camera to make reaction to the inputment of user.
    fn react_input(&mut self, actioner: &ActionNerve, delta_time: f32);
}


/// Custom implementation of the LookAt function
#[allow(dead_code)]
fn lookat_matrix(pos: &Point3<f32>, target: &Point3<f32>, world_up: &Vector3<f32>) -> Matrix4<f32> {

    // 1. Position = known
    // 2. Calculate camera direction.
    let z_axis = (pos - target).normalize();
    // 3. Get positive right axis vector.
    let x_axis = world_up.normalize().cross(&z_axis).normalize();
    // 4. Calculate camera up vector.
    let y_axis = z_axis.cross(&x_axis);

    // Create translation and rotation matrix
    // Access elements as mat[col][row] due to column-major layout
    let mut translation: Matrix4<f32> = Matrix4::identity();
    translation.set_column(3, &Vector4::new(-pos.x, -pos.y, -pos.z, 1.0));

    let rotation = Matrix4::from_rows(&[
        RowVector4::new(x_axis.x, x_axis.y, x_axis.z, 0.0),
        RowVector4::new(y_axis.x, y_axis.y, y_axis.z, 0.0),
        RowVector4::new(z_axis.x, y_axis.y, y_axis.z, 0.0),
        RowVector4::new(0.0, 0.0, 0.0, 1.0),
    ]);

    // Return lookAt matrix as combination of translation and rotation matrix.
    rotation * translation // Remember to read from right to left (first translation then rotation)
}
