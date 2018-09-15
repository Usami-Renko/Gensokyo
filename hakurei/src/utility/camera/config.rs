
use cgmath::{ Vector3, Point3 };

use utility::camera::chase::HaChaseCamera;
use utility::camera::flight::HaFlightCamera;
use utility::camera::stage::HaStageCamera;

pub struct CameraConfigurator {

    pos     : Point3<f32>,
    world_up: Vector3<f32>,

    yaw  : f32,
    pitch: f32,

    near: f32,
    far : f32,
    screen_aspect: f32,
}

impl CameraConfigurator {

    pub fn config() -> CameraConfigurator {
        CameraConfigurator::default()
    }

    pub fn place_at(mut self, position: Point3<f32>) -> Self { self.pos = position; self }
    pub fn world_up(mut self, up: Vector3<f32>) -> Self { self.world_up = up; self }
    pub fn yaw(mut self, yaw: f32) -> Self { self.yaw = yaw; self }
    pub fn pitch(mut self, pitch: f32) -> Self { self.pitch = pitch; self }
    pub fn view_distance(mut self, near: f32, far: f32) -> Self { self.near = near; self.far  = far; self }
    pub fn screen_dimension(mut self, width: u32, height: u32) -> Self { self.screen_aspect = (width as f32) / (height as f32); self }

    pub fn for_chase_camera(self) -> HaChaseCamera {
        unimplemented!()
    }

    pub fn for_flight_camera(self) -> HaFlightCamera {
        HaFlightCamera::new(self.pos, self.world_up, self.yaw, self.pitch, self.near, self.far, self.screen_aspect)
    }

    pub fn for_stage_camera(self) -> HaStageCamera {
        HaStageCamera::new(self.pos, self.world_up, self.yaw, self.pitch, self.near, self.far, self.screen_aspect)
    }
}


impl Default for CameraConfigurator {

    fn default() -> CameraConfigurator {
        CameraConfigurator {

            pos     : Point3::new(0.0, 0.0, 0.0),
            world_up: Vector3::new(0.0, 1.0, 0.0),

            yaw  : -90.0,
            pitch: 0.0,

            near: 0.1,
            far : 100.0,
            screen_aspect: 1.0,
        }
    }
}