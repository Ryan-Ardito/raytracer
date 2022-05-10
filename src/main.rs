use std::sync::Arc;

mod vec3;
use vec3::{Point3, Color};

mod camera;
use camera::Camera;
mod hittable;
use hittable::{HittableList};
mod material;
use material::{Lambertian, Metal};
mod ray;
mod sphere;
use sphere::Sphere;
mod utils;
use utils::{random_double, ray_color};

fn main() {

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 800;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;
    let max_depth:u32 = 50;

    // World
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left   = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right  = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(Sphere::new(Point3::new( 0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(Point3::new( -1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Point3::new( 1.0, 0.0, -1.0), 0.5, material_right));

    // Camera

    let cam = Camera::new();

    // Render

    print!("P3\n{image_width} {image_height}\n255\n");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j} ");
        for i in 0..image_width {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width as f64 - 1.0);
                let v = (j as f64 + random_double()) / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);
            }
            pixel_color.write_color(samples_per_pixel)
        }
    }
}