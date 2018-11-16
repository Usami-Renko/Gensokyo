
pub(crate) use self::time::{ FPS_SAMPLE_COUNT, FPS_SAMPLE_COUNT_FLOAT, DEFAULT_PREFER_FPS };
pub(crate) use self::camera::{ CAMERA_MOVE_SPEED, CAMERA_MOUSE_SENTIVITY, CAMERA_WHEEL_SENTIVITY, CAMERA_ZOOM, CAMERA_YAW, CAMERA_PITCH };

mod config;
mod camera;
mod time;
