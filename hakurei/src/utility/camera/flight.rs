
use cgmath;
use cgmath::{ Matrix4, Vector3, Point3, InnerSpace, Zero, Deg };
use winit::VirtualKeyCode;

use config::camera as CameraConfig;
use input::action::ActionNerve;

use utility::camera::traits::HaCameraAbstract;

pub struct HaFlightCamera {

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

impl HaCameraAbstract for HaFlightCamera {

    /// Return the view matrix base on the properties of camera.
    fn view_matrix(&self) -> Matrix4<f32> {

        Matrix4::look_at(self.pos, self.pos + self.front, self.up)
    }

    fn proj_matrix(&self) -> Matrix4<f32> {

        cgmath::perspective(Deg(self.zoom), self.screen_aspect, self.near, self.far)
    }

    fn reset_screen_dimension(&mut self, width: u32, height: u32) {
        self.screen_aspect = (width as f32) / (height as f32);
    }

    // TODO: This method is not complete.
    fn react_input(&mut self, actioner: &ActionNerve, delta_time: f32) {

        // keyboard
        let velocity = self.move_speed * delta_time;

        if actioner.is_key_pressed_raw(VirtualKeyCode::Up) {
            self.pos += self.front * velocity;
        } else if actioner.is_key_pressed_raw(VirtualKeyCode::Down) {
            self.pos -= self.front * velocity;
        }

        if actioner.is_key_pressed_raw(VirtualKeyCode::Left) {
            self.pos -= self.right * velocity;
        } else if actioner.is_key_pressed_raw(VirtualKeyCode::Right) {
            self.pos += self.right * velocity;
        }

    }
}

impl HaFlightCamera {

    pub(super) fn new(pos: Point3<f32>, world_up: Vector3<f32>, yaw: f32, pitch: f32, near: f32, far: f32, screen_aspect: f32) -> HaFlightCamera {
        let mut camera = HaFlightCamera {
            pos, world_up, yaw, pitch, near, far, screen_aspect,
            ..Default::default()
        };
        camera.update_vectors();
        camera
    }

    pub fn set_move_speed(&mut self, speed: f32) {
        self.move_speed = speed;
    }

    pub(super) fn update_vectors(&mut self) {
        // calculate the new front vector
        let front_x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        let front_y = self.pitch.to_radians().sin();
        let front_z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.front = Vector3::new(front_x, front_y, front_z).normalize();

        // also calculte the right and up vector.
        // Normalize the vectors, because their length gets closer to 0 the move you look up or down which results in slower movement.
        self.right = self.front.cross(self.world_up);
        self.up    = self.right.cross(self.front);
    }
}

impl Default for HaFlightCamera {

    fn default() -> HaFlightCamera {
        HaFlightCamera {
            pos  : Point3::new(0.0, 0.0, 0.0),
            front: Vector3::new(0.0, 0.0, -1.0),
            up   : Vector3::zero(),
            right: Vector3::zero(),
            world_up: Vector3::unit_y(),

            yaw  : CameraConfig::CAMERA_YAW,
            pitch: CameraConfig::CAMERA_PITCH,

            move_speed: CameraConfig::CAMERA_MOVE_SPEED,
            _mouse_sentivity: CameraConfig::CAMERA_MOUSE_SENTIVITY,
            _wheel_sentivity: CameraConfig::CAMERA_WHEEL_SENTIVITY,

            zoom: CameraConfig::CAMERA_ZOOM,
            near: 0.1,
            far : 100.0,
            screen_aspect: 1.0,
        }
    }
}
