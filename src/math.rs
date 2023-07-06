use crate::{vec3::Vec3, WINDOW_DIMENSIONS};

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
}

pub trait Collidable {
    fn find_collision_position(&self, ray: &Ray) -> Option<(Vec3, Vec3)>;
    fn find_if_collides(&self, ray: &Ray) -> bool {
        self.find_collision_position(ray).is_some()
    }
}

impl Collidable for Sphere {
    /// taken from the equations found on this linkhttp://www.ambrnet.com/TrigoCalc/Sphere/SpherLineIntersection_.htm
    fn find_collision_position(&self, ray: &Ray) -> Option<(Vec3, Vec3)> {
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
        Some((ray.calc_p(t1), ray.calc_p(t2)))
    }
}

pub fn get_vector_from_index(i: usize, width: usize, height: usize) -> Vec3 {
    let hw = (width / 2) as f32;
    let hh = (height / 2) as f32;
    let aspect_ratio = width as f32 / height as f32;
    let x = (i % width) as f32 / hw - 1.;
    let y = (i / width) as f32 / hh - 1.;
    Vec3 {
        x: x as f32 * aspect_ratio,
        y: -y as f32,
        z: 0.,
    }
}
