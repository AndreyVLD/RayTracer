#![allow(non_snake_case)]
mod camera;
mod ray;
mod shapes;
mod vector3;

use crate::shapes::{Hittable, Sphere, Surface};
use crate::vector3::Vector3;
use camera::Camera;

pub fn generate_image(width: u32, aspect_ratio: f64) {
    let camera = Camera::new(width, aspect_ratio, 100, 50);

    let mut sphere_1 = Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5));
    sphere_1.surface = Surface::new(Vector3::new(120.0, 70.0, 255.0));

    let sphere_2 = Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0));

    let hittable: Vec<Box<dyn Hittable>> = vec![sphere_1, sphere_2];
    camera.render(hittable);
}
