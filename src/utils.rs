#![allow(dead_code)]
use crate::vec3::{self, Point3, Color};
use crate::ray::Ray;
use crate::hittable::Hittable;

use rand::Rng;

const INFINITY: f64 = f64::INFINITY;
const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64{
    degrees * PI / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    return x;
}

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = vec3::dot(&oc, &ray.direction);
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return -half_b - f64::sqrt(discriminant) / a;
    }
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_double_bound(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn ray_color(r: &Ray, world: &impl Hittable, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    match world.hit(r, 0.001, INFINITY) {
        Some(rec) => {
            match rec.material.scatter(r, &rec) {
                Some((attenuation, scattered)) => return ray_color(&scattered, world, depth-1) * attenuation,
                None => return Color::new(0.0, 0.0, 0.0)
            }
        }
        None => (),
    };
    let unit_direction = vec3::unit_vector(r.direction);
    let t = 0.5*(unit_direction.y + 1.0);
    return Color::new(1.0, 1.0, 1.0) * (1.0-t) + Color::new(0.5, 0.7, 1.0) * t;
}
