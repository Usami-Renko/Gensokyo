
use crate::assets::camera_expt::target::{ GsCamera, GsCameraApi };
use crate::input::{ ActionNerve, GsKeycode };
use crate::utils::types::{ Matrix4F, Vector3F };

pub struct Flight;

impl GsCameraApi<Flight> for GsCamera<Flight> {

    fn update_view(&mut self) {

        let rotate_mat = Matrix4F::new_rotation(self.rotation);
        let translate_mat = Matrix4F::new_translation(&Vector3F::new(self.position.x, self.position.y, self.position.z));

        self.view = rotate_mat * translate_mat;
    }

    fn react_input(&mut self, actioner: &ActionNerve, delta_time: f32) {

        let camera_front = {
            let x_radian = self.rotation.x.to_radians();
            let y_radian = self.rotation.y.to_radians();

            let x = (-x_radian.cos()) * y_radian.sin();
            let y = x_radian.sin();
            let z = x_radian.cos() * y_radian.cos();

            let camera_front = Vector3F::new(x, y, z);
            nalgebra::Unit::new_normalize(camera_front).into_inner()
        };

        let move_speed = delta_time * self.move_speed;

        if actioner.is_key_pressed(GsKeycode::UP) {
            self.position += camera_front * move_speed;
        } else if actioner.is_key_pressed(GsKeycode::DOWN) {
            self.position -= camera_front * move_speed;
        }

        if actioner.is_key_pressed(GsKeycode::LEFT) {
            let direction = camera_front.cross(&Vector3F::new(0.0, 1.0, 0.0));
            self.position += nalgebra::Unit::new_normalize(direction).into_inner() * move_speed;
        } else if actioner.is_key_pressed(GsKeycode::RIGHT) {
            let direction = camera_front.cross(&Vector3F::new(0.0, 1.0, 0.0));
            self.position -= nalgebra::Unit::new_normalize(direction).into_inner() * move_speed;
        }

        self.update_view();
    }
}
