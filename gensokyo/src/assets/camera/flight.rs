
use num;
use nalgebra::{ Matrix4, Vector3, Point3, zero };

use crate::input::ActionNerve;
use crate::input::GsKeycode;

use crate::config::utils;
use crate::assets::camera::traits::GsCameraAbstract;

pub struct GsFlightCamera {

    /// Camera position.
    pos  : Point3<f32>,
    /// Front direction.
    front: Vector3<f32>,
    /// Up direction.
    up   : Vector3<f32>,
    /// right direction.
    right: Vector3<f32>,

    world_up: Vector3<f32>,

    yaw  : f32,
    pitch: f32,

    // camera options
    move_speed: f32,
    _mouse_sentivity: f32,
    _wheel_sentivity: f32,

    zoom: f32,
    near: f32,
    far : f32,
    screen_aspect: f32,
}

impl GsCameraAbstract for GsFlightCamera {

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

        // keyboard
        let velocity = self.move_speed * delta_time;

        if actioner.is_key_pressed(GsKeycode::UP) {
            self.pos += self.front * velocity;
        } else if actioner.is_key_pressed(GsKeycode::DOWN) {
            self.pos -= self.front * velocity;
        }

        if actioner.is_key_pressed(GsKeycode::LEFT) {
            self.pos -= self.right * velocity;
        } else if actioner.is_key_pressed(GsKeycode::RIGHT) {
            self.pos += self.right * velocity;
        }

        // mouse motion
        if actioner.is_mouse_active() {
            let mut mouse_motion = actioner.mouse_motion();
            mouse_motion = mouse_motion.scale(0.5);
            self.yaw += mouse_motion.delta_x;
            self.pitch = num::clamp(self.pitch - mouse_motion.delta_y, -89.0, 89.0);

            // recalculate front, right or up vector only when mouse move.
            self.update_vectors();
        }
    }
}

impl GsFlightCamera {

    pub(super) fn new(pos: Point3<f32>, world_up: Vector3<f32>, yaw: f32, pitch: f32, near: f32, far: f32, screen_aspect: f32) -> GsFlightCamera {
        let mut camera = GsFlightCamera {
            pos, world_up, yaw, pitch, near, far, screen_aspect,
            ..Default::default()
        };
        camera.update_vectors();
        camera
    }

    pub fn set_move_speed(&mut self, speed: f32) {
        self.move_speed = speed;
    }

    fn update_vectors(&mut self) {
        // calculate the new front vector.
        let front_x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        let front_y = self.pitch.to_radians().sin();
        let front_z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.front = Vector3::new(front_x, front_y, front_z).normalize();

        // also calculte the right and up vector.
        // Normalize the vectors, because their length gets closer to 0 the move you look up or down which results in slower movement.
        self.right = self.front.cross(&self.world_up);
        self.up    = self.right.cross(&self.front);
    }
}

impl Default for GsFlightCamera {

    fn default() -> GsFlightCamera {
        GsFlightCamera {
            pos  : Point3::new(0.0, 0.0, 0.0),
            front: Vector3::new(0.0, 0.0, -1.0),
            up   : zero(),
            right: zero(),
            world_up: Vector3::y(),

            yaw  : utils::CAMERA_YAW,
            pitch: utils::CAMERA_PITCH,

            move_speed: utils::CAMERA_MOVE_SPEED,
            _mouse_sentivity: utils::CAMERA_MOUSE_SENTIVITY,
            _wheel_sentivity: utils::CAMERA_WHEEL_SENTIVITY,

            zoom: utils::CAMERA_ZOOM,
            near: 0.1,
            far : 100.0,
            screen_aspect: 1.0,
        }
    }
}
