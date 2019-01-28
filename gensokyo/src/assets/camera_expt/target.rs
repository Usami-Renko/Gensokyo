
use crate::input::ActionNerve;
use crate::utils::types::{ Point3F, Vector3F, Matrix4F };

use std::marker::PhantomData;

pub struct GsCamera<C> {

    phantom_type: PhantomData<C>,

    pub(super) fovy : f32,
    pub(super) znear: f32,
    pub(super) zfar : f32,

    pub(super) position: Point3F,
    pub(super) move_speed: f32,

    pub(super) rotation: Vector3F,
    pub(super) rotate_speed: f32,

    pub(super) perspective: Matrix4F,
    pub(super) view: Matrix4F,
}

impl<C> Default for GsCamera<C> {

    fn default() -> GsCamera<C> {

        GsCamera {
            phantom_type: PhantomData,
            fovy : 0.0,
            znear: 0.0,
            zfar : 0.0,

            position: Point3F::new(0.0, 0.0, 0.0),
            move_speed: 1.0,

            rotation: nalgebra::zero(),
            rotate_speed: 1.0,

            perspective: Matrix4F::identity(),
            view: Matrix4F::identity(),
        }
    }
}

impl<C> GsCamera<C>
    where
        Self: GsCameraApi<C> {

    pub fn new(place_at: Point3F) -> GsCamera<C> {
        GsCamera {
            position: place_at,
            ..Default::default()
        }
    }

    pub fn set_move_speed(&mut self, speed: f32) {
        self.move_speed = speed;
    }

    pub fn set_rotate_speed(&mut self, speed: f32) {
        self.rotate_speed = speed;
    }

    pub fn calc_perspective(&self) -> Matrix4F {
        self.perspective.clone()
    }

    pub fn calc_view(&self) -> Matrix4F {
        self.view.clone()
    }

    pub fn reset_perspective(&mut self, fovy: f32, aspect: f32, znear: f32, zfar: f32) {

        self.fovy = fovy;
        self.znear = znear;
        self.zfar = zfar;
        self.perspective = Matrix4F::new_perspective(aspect, fovy, znear, zfar);
    }

    pub fn reset_aspect_ratio(&mut self, aspect: f32) {
        self.perspective = Matrix4F::new_perspective(aspect, self.fovy, self.znear, self.zfar);
    }

    pub fn reset_position(&mut self, position: Point3F) {
        self.position = position;
        self.update_view();
    }

    pub fn translate(&mut self, delta: &Vector3F) {
        self.position += delta;
        self.update_view();
    }

    pub fn reset_rotation(&mut self, rotation: Vector3F) {
        self.rotation = rotation;
        self.update_view();
    }

    pub fn rotate(&mut self, delta: &Vector3F) {
        self.rotation += delta;
        self.update_view();
    }
}

pub trait GsCameraApi<C> {

    fn update_view(&mut self);
    fn react_input(&mut self, actioner: &ActionNerve, delta_time: f32);
}
