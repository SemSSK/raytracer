use nalgebra::Rotation3;

use crate::{
    vec3::{ConvertableToColor, Vec3},
    WINDOW_DIMENSIONS,
};

#[derive(Debug, Default)]
pub struct Ray {
    pub position: Vec3,
    pub direction: Vec3,
}

impl Ray {
    fn calc_p(&self, t: f32) -> Vec3 {
        self.position + self.direction.scale(t)
    }
}

#[derive(Debug, Default)]
pub struct Sphere {
    pub center: Vec3,
    pub ray: f32,
    pub color: Vec3,
}

pub trait Collidable {
    fn find_collision_position(&self, ray: &Ray) -> Option<Vec3>;
    fn find_if_collides(&self, ray: &Ray) -> bool {
        self.find_collision_position(ray).is_some()
    }
    fn find_color_to_display(&self, ray: &Ray, light: &Vec3) -> Option<Vec3>;
}

impl Collidable for Sphere {
    /// taken from the equations found on this linkhttp://www.ambrnet.com/TrigoCalc/Sphere/SpherLineIntersection_.htm
    fn find_collision_position(&self, ray: &Ray) -> Option<Vec3> {
        let a = ray.direction.dot(&ray.direction);
        let b =
            ray.position.dot(&ray.direction.scale(2.)) - ray.direction.dot(&self.center.scale(2.));
        let c = self.center.dot(&self.center)
            + self.center.dot(&ray.position.scale(-2.))
            + ray.position.dot(&ray.position)
            - self.ray.powi(2);
        let delta = b.powi(2) - 4. * a * c;
        if delta < 0. {
            return None;
        }
        let t1 = (-b + delta.sqrt()) / (2. * a);
        let t2 = (-b - delta.sqrt()) / (2. * a);
        let (p1, p2) = (ray.calc_p(t1), ray.calc_p(t2));
        let (n1, n2) = (p1 - self.center, p2 - self.center);
        if n1.dot(&ray.direction) > 0. {
            Some(p2)
        } else {
            Some(p1)
        }
    }

    fn find_color_to_display(&self, ray: &Ray, light: &Vec3) -> Option<Vec3> {
        let Some(p) = self.find_collision_position(&ray) else {
            return None;
        };
        let normal = (p - self.center).normalized();
        let d = normal.dot(&light.scale(-1.)).max(0.);
        Some(self.color.scale(d).clamp())
    }
}

pub fn get_vector_from_index(
    i: usize,
    width: usize,
    height: usize,
    camera: &(Vec3, Rotation3<f32>),
) -> Vec3 {
    let hw = (width / 2) as f32;
    let hh = (height / 2) as f32;
    let aspect_ratio = width as f32 / height as f32;
    let x = (i % width) as f32 / hw - 1.;
    let y = (i / width) as f32 / hh - 1.;
    let res = camera.1 * (nalgebra::Vector3::new(x as f32 * aspect_ratio, -y as f32, 5.))
        + camera.0.to_vec3();
    Vec3::from_vec3(&res)
}
