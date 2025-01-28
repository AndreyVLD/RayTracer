mod camera;
pub mod hit;
pub mod material;
mod ray;
mod shapes;
mod texture;
pub mod transformation;
mod utils;
mod vector3;

use crate::hit::Hittable;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::shapes::box_quad::BoxQuad;
use crate::shapes::quad::Quad;
use crate::shapes::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture};
use crate::transformation::{RotateY, Translate};
use crate::utils::background_gradient;
use crate::vector3::Vector3;
use camera::Camera;
use std::io::{self, Read};
use std::sync::Arc;
use std::time::Instant;

pub fn spheres() {
    let camera = Camera::new(
        1920,
        16.0 / 9.0,
        20,
        10,
        background_gradient,
        20.0,
        Vector3::new(13.0, 2.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        0.2,
        10.0,
    );
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let checker = Box::new(CheckerTexture::new(
        3.0,
        Vector3::new(0.2, 0.3, 0.1),
        Vector3::new(0.9, 0.9, 0.9),
    ));

    let material_ground = Arc::new(Lambertian::from_texture(checker));
    world.push(Box::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = fastrand::f64();
            let center = Vector3::new(
                a as f64 + 0.9 * fastrand::f64(),
                0.2,
                b as f64 + 0.9 * fastrand::f64(),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Arc<dyn Material>;
                match choose_mat {
                    0.0..0.8 => {
                        // diffuse
                        let albdeo = Vector3::random(0.0, 1.0) * Vector3::random(0.0, 1.0);
                        material = Arc::new(Lambertian::new(albdeo));
                        world.push(Box::new(Sphere::new(center, 0.2, material)));
                    }
                    0.8..0.95 => {
                        // metal
                        let albedo = Vector3::random(0.5, 1.0);
                        let fuzz = fastrand::f64() * 0.5;
                        material = Arc::new(Metal::new(albedo, fuzz));
                        world.push(Box::new(Sphere::new(center, 0.2, material)));
                    }
                    _ => {
                        // glass
                        material = Arc::new(Dielectric::new(1.5));
                        world.push(Box::new(Sphere::new(center, 0.2, material)));
                    }
                }
            }
        }
    }
    let material_1 = Arc::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Arc::new(Lambertian::new(Vector3::new(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Arc::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    camera.render(world);
}

fn checkered_spheres() {
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    let checker_1 = Box::new(CheckerTexture::new(
        3.0,
        Vector3::new(0.2, 0.3, 0.1),
        Vector3::new(0.9, 0.9, 0.9),
    ));

    let checker_2 = Box::new(CheckerTexture::new(
        3.0,
        Vector3::new(0.2, 0.3, 0.1),
        Vector3::new(0.9, 0.9, 0.9),
    ));

    world.push(Box::new(Sphere::new(
        Vector3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from_texture(checker_1)),
    )));

    world.push(Box::new(Sphere::new(
        Vector3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from_texture(checker_2)),
    )));

    let camera = Camera::new(
        400,
        16.0 / 9.0,
        100,
        50,
        background_gradient,
        20.0,
        Vector3::new(13.0, 2.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        0.0,
        0.0,
    );

    camera.render(world);
}

fn earth() {
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    let earth_texture = Box::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));

    world.push(Box::new(Sphere::new(
        Vector3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    )));

    let camera = Camera::new(
        400,
        16.0 / 9.0,
        100,
        50,
        background_gradient,
        20.0,
        Vector3::new(0.0, 0.0, 12.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        0.0,
        0.0,
    );

    camera.render(world);
}

fn quads() {
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    // Materials
    let left_red = Arc::new(Lambertian::new(Vector3::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Vector3::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(Vector3::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Vector3::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Vector3::new(0.2, 0.8, 0.8)));

    //Quads
    world.push(Box::new(Quad::new(
        Vector3::new(-3.0, -2.0, 5.0),
        Vector3::new(0.0, 0.0, -4.0),
        Vector3::new(0.0, 4.0, 0.0),
        left_red,
    )));

    world.push(Box::new(Quad::new(
        Vector3::new(-2.0, -2.0, 0.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 4.0, 0.0),
        back_green,
    )));

    world.push(Box::new(Quad::new(
        Vector3::new(3.0, -2.0, 1.0),
        Vector3::new(0.0, 0.0, 4.0),
        Vector3::new(0.0, 4.0, 0.0),
        right_blue,
    )));

    world.push(Box::new(Quad::new(
        Vector3::new(-2.0, 3.0, 1.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));

    world.push(Box::new(Quad::new(
        Vector3::new(-2.0, -3.0, 5.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    // Camera
    let camera = Camera::new(
        400,
        1.0,
        100,
        50,
        background_gradient,
        80.0,
        Vector3::new(0.0, 0.0, 9.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        0.0,
        1.0,
    );
    camera.render(world);
}

fn simple_lights() {
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    let material = Arc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        material.clone(),
    )));

    world.push(Box::new(Sphere::new(
        Vector3::new(0.0, 2.0, 0.0),
        2.0,
        material,
    )));

    let diff_light = Arc::new(DiffuseLight::new(Vector3::new(4.0, 4.0, 4.0)));
    world.push(Box::new(Sphere::new(
        Vector3::new(0.0, 7.0, 0.0),
        2.0,
        diff_light.clone(),
    )));

    world.push(Box::new(Quad::new(
        Vector3::new(3.0, 1.0, -2.0),
        Vector3::new(2.0, 0.0, 0.0),
        Vector3::new(0.0, 2.0, 0.0),
        diff_light,
    )));

    let camera = Camera::new(
        400,
        16.0 / 9.0,
        100,
        50,
        |_| Vector3::new(0.0, 0.0, 0.0),
        20.0,
        Vector3::new(26.0, 3.0, 6.0),
        Vector3::new(0.0, 2.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        0.0,
        0.0,
    );

    camera.render(world);
}

fn cornell_box() {
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let red = Arc::new(Lambertian::new(Vector3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Vector3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vector3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Vector3::new(15.0, 15.0, 15.0)));

    world.push(Box::new(Quad::new(
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        green,
    )));

    world.push(Box::new(Quad::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        red,
    )));

    world.push(Box::new(Quad::new(
        Vector3::new(343.0, 554.0, 332.0),
        Vector3::new(-130.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -105.0),
        light,
    )));

    world.push(Box::new(Quad::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.push(Box::new(Quad::new(
        Vector3::new(555.0, 555.0, 555.0),
        Vector3::new(-555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    world.push(Box::new(Quad::new(
        Vector3::new(0.0, 0.0, 555.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box_1: Arc<dyn Hittable> = Arc::new(BoxQuad::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));

    box_1 = Arc::new(RotateY::new(box_1, 15.0));

    world.push(Box::new(Translate::new(
        box_1,
        Vector3::new(265.0, 0.0, 295.0),
    )));

    let mut box_2: Arc<dyn Hittable> = Arc::new(BoxQuad::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));

    box_2 = Arc::new(RotateY::new(box_2, -18.0));
    world.push(Box::new(Translate::new(
        box_2,
        Vector3::new(130.0, 0.0, 65.0),
    )));

    let camera = Camera::new(
        600,
        1.0,
        200,
        50,
        |_| Vector3::new(0.0, 0.0, 0.0),
        40.0,
        Vector3::new(278.0, 278.0, -800.0),
        Vector3::new(278.0, 278.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        0.0,
        0.0,
    );
    camera.render(world);
}

fn main() {
    let now = Instant::now();

    // Scenes to be rendered
    match 6 {
        1 => spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => quads(),
        5 => simple_lights(),
        6 => cornell_box(),
        _ => {}
    }

    println!(
        "Time elapsed in generate image: {} ms",
        now.elapsed().as_millis()
    );

    println!("Press any key to close...");
    let mut buffer = [0; 1];
    let _ = io::stdin().read(&mut buffer);
}
