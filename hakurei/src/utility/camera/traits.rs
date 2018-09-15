
use cgmath::{ Matrix4, Point3, Vector3, SquareMatrix, InnerSpace };
use input::action::ActionNerve;

pub trait HaCameraAbstract {

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
    let x_axis = world_up.normalize().cross(z_axis).normalize();
    // 4. Calculate camera up vector.
    let y_axis = z_axis.cross(x_axis);

    // Create translation and rotation matrix
    // Access elements as mat[col][row] due to column-major layout
    let mut translation: Matrix4<f32> = Matrix4::identity();
    translation[3][0] = -pos.x;
    translation[3][1] = -pos.y;
    translation[3][2] = -pos.z;

    let mut rotation: Matrix4<f32> = Matrix4::identity();
    rotation[0][0] = x_axis.x; // First column, first row.
    rotation[1][0] = x_axis.y;
    rotation[2][0] = x_axis.z;
    rotation[0][1] = y_axis.x; // First column, second row.
    rotation[1][1] = y_axis.y;
    rotation[2][1] = y_axis.z;
    rotation[0][2] = z_axis.x; // First column, third row.
    rotation[1][2] = z_axis.y;
    rotation[2][2] = z_axis.z;

    // Return lookAt matrix as combination of translation and rotation matrix.
    rotation * translation // Remember to read from right to left (first translation then rotation)
}
