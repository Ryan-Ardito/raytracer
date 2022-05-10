#![allow(dead_code)]
use crate::utils::{clamp, random_double, random_double_bound};

#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}\n",
            (255.999 * self.x) as u16,
            (255.999 * self.y) as u16,
            (255.999 * self.z) as u16,
        )
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {
            x, y, z,
        }
    }

    pub fn negative(&self) -> Vec3 { Vec3::new(-self.x, -self.y, -self.z) }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn random() -> Vec3 {
        return Vec3::new(random_double(), random_double(), random_double());
    }

    pub fn random_bound(min: f64, max: f64) -> Vec3 {
        return Vec3::new(random_double_bound(min, max), random_double_bound(min, max), random_double_bound(min, max));
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_bound(-1.0, 1.0);
            if p.length_squared() >= 1.0 { continue; }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - (*n * dot(v,n) * 2.0)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.x + v.x,
            self.y + v.y,
            self.z + v.z,
        )
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.x - v.x,
            self.y - v.y,
            self.z - v.z,
        )
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        self * (1.0 / t)
    }
}

// Type aliases for vec3
pub type Point3 = Vec3;
pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, samples_per_pixel: u32) {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;

        // Divide the color by the number of samples and gamma-correct for gamma=2.0
        let scale = 1.0 / samples_per_pixel as f64;
        r = f64::sqrt(scale * r);
        g = f64::sqrt(scale * g);
        b = f64::sqrt(scale * b);

        // Write the translated [0,255] value of each color component.
        print!(
            "{} {} {}\n",
            (256.0 * clamp(r, 0.0, 0.999)) as u16,
            (256.0 * clamp(g, 0.0, 0.999)) as u16,
            (256.0 * clamp(b, 0.0, 0.999)) as u16);
    }
}

// vec3 Utility Functions

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}