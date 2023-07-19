use nalgebra::{Rotation3, Unit, Vector3};

pub struct HitPayload<'a> {
    position: Vector3<f32>,
    distance: f32,
    normal_unnormalized: Vector3<f32>,
    collidable: &'a dyn Collidable,
}

impl HitPayload<'_> {
    fn get_normal(&self) -> Unit<Vector3<f32>> {
        Unit::new_normalize(self.normal_unnormalized)
    }
}

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
        scene: &[Sphere],
        light: &Vector3<f32>,
        ambiant: f32,
        bounces: u32,
    ) -> Option<Vector3<f32>> {
        if bounces == 0 {
            return None;
        }
        let Some((new_ray,color)) = scene
            .iter()
            .map(|collidable| collidable.find_collision_position(self))
            .fold(
                (f32::INFINITY, None),
                |acc, option_hit_payload| match option_hit_payload {
                    Some(hit_payload) => {
                        if hit_payload.distance < acc.0 {
                            (hit_payload.distance, Some(hit_payload))
                        } else {
                            acc
                        }
                    }
                    None => acc,
                },
            )
            .1
            .and_then(|hit_payload| {
                Some((
                    Ray {
                        direction: hit_payload.normal_unnormalized,
                        position: hit_payload.position + hit_payload.normal_unnormalized.scale(0.01)
                    },hit_payload
                        .collidable
                        .find_color_to_display(hit_payload, &light, ambiant)
                ))
            }) else {
                return None;
            };
        let sum_color = match new_ray.cast(scene, light, ambiant, bounces - 1) {
            Some(color) => color,
            None => Vector3::zeros(),
        };
        Some(color + sum_color)
    }
}

#[derive(Debug, Default)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub ray: f32,
    pub color: Vector3<f32>,
}

pub trait Collidable {
    fn find_collision_position(&self, ray: &Ray) -> Option<HitPayload>;
    fn find_if_collides(&self, ray: &Ray) -> bool {
        self.find_collision_position(ray).is_some()
    }
    fn find_color_to_display(
        &self,
        col_point: HitPayload,
        light: &Vector3<f32>,
        ambiant: f32,
    ) -> Vector3<f32>;
}

impl Collidable for Sphere {
    /// taken from the equations found on this linkhttp://www.ambrnet.com/TrigoCalc/Sphere/SpherLineIntersection_.htm
    fn find_collision_position(&self, ray: &Ray) -> Option<HitPayload> {
        if ray.direction.dot(&(self.center - ray.position)) < 0. {
            return None;
        }
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
        let p = if n1.dot(&ray.direction) > 0. { p2 } else { p1 };
        Some(HitPayload {
            collidable: self,
            distance: (p - ray.position).magnitude_squared(),
            normal_unnormalized: p - self.center,
            position: p,
        })
    }

    fn find_color_to_display(
        &self,
        hit_payload: HitPayload,
        light: &Vector3<f32>,
        ambiant: f32,
    ) -> Vector3<f32> {
        let p = hit_payload.position;
        let normal = hit_payload.get_normal();
        let light = Unit::new_normalize(p - light).scale(-1.);
        let d_light = normal.dot(&light).max(0.);
        self.color.scale(d_light + ambiant)
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

    camera.1 * (nalgebra::Vector3::new(x * aspect_ratio, -y, 5.)) + camera.0
}
