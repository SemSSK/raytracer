use nalgebra::Rotation3;

use crate::{math::Ray, vec3::Vec3};

#[derive(Debug)]
pub struct CameraTransform {
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub trans_x: f32,
    pub trans_y: f32,
    pub trans_z: f32,
}

impl Default for CameraTransform {
    fn default() -> Self {
        Self {
            rot_x: 0.,
            rot_y: 0.,
            rot_z: 0.,
            trans_x: 0.,
            trans_y: 0.,
            trans_z: 0.,
        }
    }
}

impl CameraTransform {
    pub fn update(&self, (position, direction): &(Vec3, Rotation3<f32>)) -> (Vec3, Rotation3<f32>) {
        let rot_x = nalgebra::Rotation3::from_axis_angle(&nalgebra::Vector3::x_axis(), self.rot_x);
        let rot_y = nalgebra::Rotation3::from_axis_angle(&nalgebra::Vector3::y_axis(), self.rot_y);
        let rot_z = nalgebra::Rotation3::from_axis_angle(&nalgebra::Vector3::z_axis(), self.rot_z);

        let position = nalgebra::Vector3::new(self.trans_x, self.trans_y, self.trans_z);
        let direction = rot_x * rot_y * rot_z;
        (Vec3::from_vec3(&position), direction)
    }
}
