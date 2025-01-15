mod camera;
mod ray;
mod shapes;
mod utils;
mod vector3;

use crate::shapes::{Hittable, Sphere};
use crate::vector3::Vector3;
use camera::Camera;
use std::time::Instant;

pub fn generate_image(width: u32, aspect_ratio: f64) {
    let camera = Camera::new(width, aspect_ratio, 100, 50);

    let sphere_1 = Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5));

    let sphere_2 = Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0));

    let hittable: Vec<Box<dyn Hittable>> = vec![sphere_1, sphere_2];
    camera.render(hittable);
}

fn main() {
    const WIDTH: u32 = 400;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    let now = Instant::now();
    generate_image(WIDTH, ASPECT_RATIO);
    println!(
        "Time elapsed in generate image: {} ms",
        now.elapsed().as_millis()
    );
}
