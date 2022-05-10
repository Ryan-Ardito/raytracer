#![allow(dead_code)]

use std::sync::Arc;
use crate::vec3::{self, Point3, Vec3};
use crate::ray::Ray;
use crate::material::Material;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(&ray.direction, &outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => outward_normal,
            false => outward_normal * -1.0,
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        let objects = Vec::new();
        HittableList {
            objects
        }
    }

    pub fn clear(mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Arc::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut record = None;

        for object in &self.objects {
            record = match object.hit(ray, t_min, closest_so_far) {
                Some(rec) => {
                    closest_so_far = rec.t;
                    Some(rec)
                },
                None => record,
            };
        }
        record
    }
}
