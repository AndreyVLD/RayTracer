mod camera;
mod ray;
mod shapes;
mod utils;
mod vector3;

use crate::shapes::{Hittable, Lambertian, Metal, Sphere};
use crate::vector3::Vector3;
use camera::Camera;
use std::time::Instant;

pub fn generate_image(width: u32, aspect_ratio: f64) {
    let camera = Camera::new(width, aspect_ratio, 100, 50);

    let material_ground = Box::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5)));
    let material_left = Box::new(Metal::new(Vector3::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Box::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0));

    let sphere_1 = Box::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));

    let sphere_2 = Box::new(Sphere::new(
        Vector3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));

    let sphere_3 = Box::new(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));

    let sphere_4 = Box::new(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    let hittable: Vec<Box<dyn Hittable>> = vec![sphere_1, sphere_2, sphere_3, sphere_4];
    camera.render(hittable);
}

fn main() {
    const WIDTH: u32 = 1920;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    let now = Instant::now();
    generate_image(WIDTH, ASPECT_RATIO);
    println!(
        "Time elapsed in generate image: {} ms",
        now.elapsed().as_millis()
    );
}
