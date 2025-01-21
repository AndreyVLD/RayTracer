mod camera;
mod ray;
mod shapes;
mod utils;
mod vector3;

use crate::shapes::{Dielectric, Hittable, Lambertian, Material, Metal, Sphere};
use crate::vector3::Vector3;
use camera::Camera;
use rand::{random, Rng};
use std::time::Instant;

pub fn generate_image() {
    let camera = Camera::new(
        1920,
        16.0 / 9.0,
        20,
        10,
        20.0,
        Vector3::new(13.0, 2.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        0.2,
        10.0,
    );

    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    let material_ground = Box::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    let mut rng = rand::rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random::<f64>();
            let center = Vector3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Box<dyn Material>;
                match choose_mat {
                    0.0..0.8 => {
                        // diffuse
                        let albdeo = Vector3::random(0.0, 1.0) * Vector3::random(0.0, 1.0);
                        material = Box::new(Lambertian::new(albdeo));
                        world.push(Box::new(Sphere::new(center, 0.2, material)));
                    }
                    0.8..0.95 => {
                        // metal
                        let albedo = Vector3::random(0.5, 1.0);
                        let fuzz = rng.random_range(0.0..0.5);
                        material = Box::new(Metal::new(albedo, fuzz));
                        world.push(Box::new(Sphere::new(center, 0.2, material)));
                    }
                    _ => {
                        // glass
                        material = Box::new(Dielectric::new(1.5));
                        world.push(Box::new(Sphere::new(center, 0.2, material)));
                    }
                }
            }
        }
    }
    let material_1 = Box::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Box::new(Lambertian::new(Vector3::new(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Box::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    camera.render(world);
}

fn main() {
    let now = Instant::now();
    generate_image();
    println!(
        "Time elapsed in generate image: {} ms",
        now.elapsed().as_millis()
    );
}
