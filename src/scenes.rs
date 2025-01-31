use crate::camera::Camera;
use crate::hit::Hittable;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::shapes::box_quad::BoxQuad;
use crate::shapes::quad::Quad;
use crate::shapes::sphere::Sphere;
use crate::shapes::volume::ConstantMedium;
use crate::texture::{CheckerTexture, ImageTexture};
use crate::transformation::{RotateY, Translate};
use crate::utils::background_gradient;
use crate::vector3::Vector3;
use fastrand::f64;
use std::sync::Arc;

/// Creates a scene with multiple spheres of different materials and renders it using the camera.
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
            let choose_mat = f64();
            let center = Vector3::new(a as f64 + 0.9 * f64(), 0.2, b as f64 + 0.9 * f64());

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
                        let fuzz = f64() * 0.5;
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

/// Creates a scene with two checkered spheres and renders it using the camera.
pub fn checkered_spheres() {
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

/// Creates a scene with a sphere textured with an image of the Earth and renders it using the camera.
pub fn earth() {
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

/// Create a scene with 4 quads and renders it using the camera.
pub fn quads() {
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

/// Creates a scene with a sphere and a quad with light material and renders it using the camera.
pub fn simple_lights() {
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
        1920,
        16.0 / 9.0,
        10000,
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

/// Creates a Cornell box scene and renders it using the camera.
pub fn cornell_box() {
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
        1920,
        1.0,
        10000,
        5,
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

/// Creates a Cornell box scene with 2 boxes made out of smoke and renders it using the camera.
pub fn cornell_smoke() {
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let red = Arc::new(Lambertian::new(Vector3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Vector3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vector3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Vector3::new(7.0, 7.0, 7.0)));

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
        Vector3::new(113.0, 554.0, 127.0),
        Vector3::new(330.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 305.0),
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
    let box_1 = Translate::new(box_1, Vector3::new(265.0, 0.0, 295.0));
    let fog_1 = ConstantMedium::new(Box::new(box_1), 0.01, Vector3::new(0.0, 0.0, 0.0));
    world.push(Box::new(fog_1));

    let mut box_2: Arc<dyn Hittable> = Arc::new(BoxQuad::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));

    box_2 = Arc::new(RotateY::new(box_2, -18.0));
    let box_2 = Translate::new(box_2, Vector3::new(130.0, 0.0, 65.0));
    let fog_2 = ConstantMedium::new(Box::new(box_2), 0.01, Vector3::new(1.0, 1.0, 1.0));
    world.push(Box::new(fog_2));

    let camera = Camera::new(
        1920,
        1.0,
        10000,
        5,
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

/// Creates the final scene with various objects and materials, and renders it using the camera.
///
/// # Arguments
///
/// * `image_width` - The width of the image in pixels.
/// * `samples` - The number of samples per pixel.
/// * `max_depth` - The maximum depth for ray tracing.
/// * `reduced` - A boolean flag to reduce the number of objects in the scene for faster rendering.
pub fn final_scene(image_width: u32, samples: u32, max_depth: u32, reduced: bool) {
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let ground = Arc::new(Lambertian::new(Vector3::new(0.48, 0.83, 0.53)));

    let boxes_per_side = if reduced { 5 } else { 20 };

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0 * (20.0 / boxes_per_side as f64);
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = f64() * 100.0 + 1.0;
            let z1 = z0 + w;

            world.push(Box::new(BoxQuad::new(
                Vector3::new(x0, y0, z0),
                Vector3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let light = Arc::new(DiffuseLight::new(Vector3::new(7.0, 7.0, 7.0)));
    world.push(Box::new(Quad::new(
        Vector3::new(123.0, 554.0, 147.0),
        Vector3::new(300.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 265.0),
        light,
    )));

    let center = Vector3::new(400.0, 400.0, 200.0);
    let sphere_material = Arc::new(Lambertian::new(Vector3::new(0.7, 0.3, 0.1)));

    world.push(Box::new(Sphere::new(center, 50.0, sphere_material)));
    world.push(Box::new(Sphere::new(
        Vector3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));

    world.push(Box::new(Sphere::new(
        Vector3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Vector3::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let mut boundary = Box::new(Sphere::new(
        Vector3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));

    world.push(boundary);

    world.push(Box::new(ConstantMedium::new(
        Box::new(Sphere::new(
            Vector3::new(360.0, 150.0, 145.0),
            70.0,
            Arc::new(Dielectric::new(1.5)),
        )),
        0.02,
        Vector3::new(0.2, 0.4, 0.9),
    )));

    boundary = Box::new(Sphere::new(
        Vector3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.push(Box::new(ConstantMedium::new(
        boundary,
        0.0001,
        Vector3::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian::from_texture(Box::new(ImageTexture::new(
        "earthmap.jpg",
    ))));

    world.push(Box::new(Sphere::new(
        Vector3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));

    let mirror = Arc::new(Metal::new(Vector3::new(0.8, 0.8, 0.8), 0.0));
    world.push(Box::new(Sphere::new(
        Vector3::new(220.0, 280.0, 300.0),
        80.0,
        mirror,
    )));

    if !reduced {
        let white = Arc::new(Lambertian::new(Vector3::new(0.73, 0.73, 0.73)));
        let ns = 1000;

        for _ in 0..ns {
            let sphere = Sphere::new(Vector3::random(0.0, 165.0), 10.0, white.clone());
            let rotate = RotateY::new(Arc::new(sphere), 15.0);
            let translate = Translate::new(Arc::new(rotate), Vector3::new(-100.0, 270.0, 395.0));
            world.push(Box::new(translate));
        }
    }

    let camera = Camera::new(
        image_width,
        1.0,
        samples,
        max_depth,
        |_| Vector3::new(0.0, 0.0, 0.0),
        40.0,
        Vector3::new(478.0, 278.0, -600.0),
        Vector3::new(278.0, 278.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        0.0,
        0.0,
    );

    camera.render(world);
}
