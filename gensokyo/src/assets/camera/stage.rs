
use nalgebra::{ Matrix4, Vector3, Point3, zero };

use crate::input::ActionNerve;
use crate::input::GsKeycode;

use crate::config::utils;
use crate::assets::camera::traits::GsCameraAbstract;

pub struct GsStageCamera {

    /// Camera position.
    pos  : Point3<f32>,
    /// Front direction.
    front: Vector3<f32>,
    /// Up direction.
    up   : Vector3<f32>,
    /// right direction.
    right: Vector3<f32>,

    // camera options
    _wheel_sentivity: f32,

    zoom: f32,
    near: f32,
    far : f32,
    screen_aspect: f32,

    // object transformation
    rotate_sensitive : f32,
    horizontal_rotate: f32,
    vertical_rotate  : f32,
}

impl GsCameraAbstract for GsStageCamera {

    fn current_position(&self) -> Point3<f32> {
        self.pos.clone()
    }

    fn view_matrix(&self) -> Matrix4<f32> {

        Matrix4::look_at_rh(&self.pos, &(self.pos + self.front), &self.up)
    }

    fn proj_matrix(&self) -> Matrix4<f32> {

        Matrix4::new_perspective(self.screen_aspect, self.zoom, self.near, self.far)
    }

    fn reset_screen_dimension(&mut self, width: u32, height: u32) {
        self.screen_aspect = (width as f32) / (height as f32);
    }

    fn react_input(&mut self, actioner: &ActionNerve, delta_time: f32) {

        if actioner.is_key_pressed(GsKeycode::UP) {
            self.vertical_rotate -= delta_time * self.rotate_sensitive;
        } else if actioner.is_key_pressed(GsKeycode::DOWN) {
            self.vertical_rotate += delta_time * self.rotate_sensitive;
        }

        if actioner.is_key_pressed(GsKeycode::RIGHT) {
            self.horizontal_rotate -= delta_time * self.rotate_sensitive;
        } else if actioner.is_key_pressed(GsKeycode::LEFT) {
            self.horizontal_rotate += delta_time * self.rotate_sensitive;
        }

        // mouse motion
        if actioner.is_mouse_active() {
            let mut mouse_motion = actioner.mouse_motion();
            mouse_motion = mouse_motion.scale(0.5);
            self.horizontal_rotate += mouse_motion.delta_x * delta_time * self.rotate_sensitive;
            self.vertical_rotate   += mouse_motion.delta_y * delta_time * self.rotate_sensitive;
        }
    }
}

impl GsStageCamera {

    pub(super) fn new(pos: Point3<f32>, world_up: Vector3<f32>, yaw: f32, pitch: f32, near: f32, far: f32, screen_aspect: f32) -> GsStageCamera {
        let mut camera = GsStageCamera {
            pos, near, far, screen_aspect,
            ..Default::default()
        };

        // calculate the new front vector
        camera.update_vectors(world_up, yaw, pitch);
        camera
    }

    /// Set the speed for rotate operation.
    ///
    /// degree_per_second means the amount of degrees in rotation when pressing the key for 1 second.
    pub fn set_rotate_speed(&mut self, degree_per_second: f32) {
        self.rotate_sensitive = degree_per_second.to_radians();
    }

    pub fn object_model_transformation(&self) -> Matrix4<f32> {

        Matrix4::from_axis_angle(&Vector3::y_axis(), self.horizontal_rotate) * Matrix4::from_axis_angle(&Vector3::x_axis(), self.vertical_rotate)
    }

    fn update_vectors(&mut self, world_up: Vector3<f32>, yaw: f32, pitch: f32) {
        // calculate the new front vector
        let front_x = yaw.to_radians().cos() * pitch.to_radians().cos();
        let front_y = pitch.to_radians().sin();
        let front_z = yaw.to_radians().sin() * pitch.to_radians().cos();

        self.front = Vector3::new(front_x, front_y, front_z).normalize();

        // also calculate the right and up vector.
        // Normalize the vectors, because their length gets closer to 0 the move you look up or down which results in slower movement.
        self.right = self.front.cross(&world_up);
        self.up    = self.right.cross(&self.front);
    }
}

impl Default for GsStageCamera {

    fn default() -> GsStageCamera {
        GsStageCamera {
            pos  : Point3::new(0.0, 0.0, 0.0),
            front: Vector3::new(0.0, 0.0, -1.0),
            up   : zero(),
            right: zero(),

            _wheel_sentivity: utils::CAMERA_WHEEL_SENTIVITY,

            zoom: utils::CAMERA_ZOOM,
            near: 0.1,
            far : 100.0,
            screen_aspect: 1.0,

            rotate_sensitive : 90.0_f32.to_radians(),
            horizontal_rotate: 0.0,
            vertical_rotate  : 0.0,
        }
    }
}
