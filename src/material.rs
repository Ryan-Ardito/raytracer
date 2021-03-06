use crate::vec3::{self, Color, Vec3};
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = Vec3::random_unit_vector() + rec.normal;

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal {
            albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&vec3::unit_vector(r_in.direction), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        let attenuation = self.albedo;
        match vec3::dot(&scattered.direction, &rec.normal) > 0.0 {
            true => Some((attenuation, scattered)),
            false => None,
        }
    }
}
