
use nalgebra::{ Vector3, Point3 };

use crate::assets::camera::chase::GsChaseCamera;
use crate::assets::camera::flight::GsFlightCamera;
use crate::assets::camera::stage::GsStageCamera;

pub struct GsCameraFactory {

    pos     : Point3<f32>,
    world_up: Vector3<f32>,

    yaw  : f32,
    pitch: f32,

    near: f32,
    far : f32,
    screen_aspect: f32,
}

impl GsCameraFactory {

    pub fn config() -> GsCameraFactory {
        GsCameraFactory::default()
    }

    pub fn place_at(mut self, position: Point3<f32>) -> GsCameraFactory {
        self.pos = position;
        self
    }

    pub fn world_up(mut self, up: Vector3<f32>) -> GsCameraFactory {
        self.world_up = up;
        self
    }

    pub fn yaw(mut self, yaw: f32) -> GsCameraFactory {
        self.yaw = yaw;
        self
    }

    pub fn pitch(mut self, pitch: f32) -> GsCameraFactory {
        self.pitch = pitch;
        self
    }
    pub fn view_distance(mut self, near: f32, far: f32) -> GsCameraFactory {
        self.near = near; self.far = far;
        self
    }

    pub fn screen_aspect_ratio(mut self, ratio: f32) -> GsCameraFactory {
        self.screen_aspect = ratio;
        self
    }

    pub fn into_chase_camera(self) -> GsChaseCamera {
        unimplemented!()
    }

    pub fn into_flight_camera(self) -> GsFlightCamera {
        GsFlightCamera::new(self.pos, self.world_up, self.yaw, self.pitch, self.near, self.far, self.screen_aspect)
    }

    pub fn into_stage_camera(self) -> GsStageCamera {
        GsStageCamera::new(self.pos, self.world_up, self.yaw, self.pitch, self.near, self.far, self.screen_aspect)
    }
}


impl Default for GsCameraFactory {

    fn default() -> GsCameraFactory {
        GsCameraFactory {

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
