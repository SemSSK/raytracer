use nalgebra::{Rotation3, SimdPartialOrd, Unit, Vector3};

#[derive(Debug, Default)]
pub struct Ray {
    pub position: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    fn calc_p(&self, t: f32) -> Vector3<f32> {
        self.position + self.direction.scale(t)
    }
    pub fn cast(
        &self,
        scene: &Vec<Sphere>,
        light: &Vector3<f32>,
        ambiant: f32,
    ) -> Option<Vector3<f32>> {
        scene
            .into_iter()
            .map(|collidable| {
                let Some(p) = collidable.find_collision_position(&self) else {
                    return None;
                };
                let color = collidable.find_color_to_display(&p, light, ambiant);
                Some((p, color))
            })
            .fold((f32::INFINITY, None), |acc, col_p| match col_p {
                Some((p, col)) => {
                    let distance = (p - self.position).dot(&(p - self.position));
                    if distance < acc.0 {
                        (distance, Some(col))
                    } else {
                        acc
                    }
                }
                None => acc,
            })
            .1
    }
}

#[derive(Debug, Default)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub ray: f32,
    pub color: Vector3<f32>,
}

pub trait Collidable {
    fn find_collision_position(&self, ray: &Ray) -> Option<Vector3<f32>>;
    fn find_if_collides(&self, ray: &Ray) -> bool {
        self.find_collision_position(ray).is_some()
    }
    fn find_color_to_display(
        &self,
        col_point: &Vector3<f32>,
        light: &Vector3<f32>,
        ambiant: f32,
    ) -> Vector3<f32>;
}

impl Collidable for Sphere {
    /// taken from the equations found on this linkhttp://www.ambrnet.com/TrigoCalc/Sphere/SpherLineIntersection_.htm
    fn find_collision_position(&self, ray: &Ray) -> Option<Vector3<f32>> {
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
        let n1 = p1 - self.center;
        if n1.dot(&ray.direction) > 0. {
            Some(p2)
        } else {
            Some(p1)
        }
    }

    fn find_color_to_display(
        &self,
        light: &Vector3<f32>,
        col_point: &Vector3<f32>,
        ambiant: f32,
    ) -> Vector3<f32> {
        let p = col_point;
        let sun = Vector3::new(-0.5, -0.5, -0.5);
        let light = Unit::new_normalize(p - light);
        let normal = Unit::new_normalize(p - self.center);
        let d = normal.dot(&light.scale(-1.)).max(0.);
        let d2 = normal.dot(&sun.scale(-1.)).max(0.);
        self.color
            .scale(d + ambiant + d2)
            .simd_clamp(Vector3::new(0., 0., 0.), Vector3::new(1., 1., 1.))
    }
}

pub fn get_vector_from_index(
    i: usize,
    width: usize,
    height: usize,
    camera: &(Vector3<f32>, Rotation3<f32>),
) -> Vector3<f32> {
    let hw = (width / 2) as f32;
    let hh = (height / 2) as f32;
    let aspect_ratio = width as f32 / height as f32;
    let x = (i % width) as f32 / hw - 1.;
    let y = (i / width) as f32 / hh - 1.;
    let res =
        camera.1 * (nalgebra::Vector3::new(x as f32 * aspect_ratio, -y as f32, 5.)) + camera.0;
    res
}
