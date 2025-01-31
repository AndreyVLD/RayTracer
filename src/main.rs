mod camera;
pub mod hit;
pub mod material;
mod ray;
mod scenes;
mod shapes;
mod texture;
pub mod transformation;
mod utils;
mod vector3;

use crate::scenes::{
    checkered_spheres, cornell_box, cornell_smoke, earth, final_scene, quads, simple_lights,
    spheres,
};
use std::io::{self, Read};
use std::time::Instant;

/// Main function
fn main() {
    let now = Instant::now();

    // Scenes to be rendered
    match 8 {
        1 => spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => quads(),
        5 => simple_lights(),
        6 => cornell_box(),
        7 => cornell_smoke(),
        8 => final_scene(1920, 10000, 5, true),
        _ => final_scene(400, 250, 4, true),
    }

    println!(
        "Time elapsed in generate image: {} ms",
        now.elapsed().as_millis()
    );

    println!("Press any key to close...");
    let mut buffer = [0; 1];
    let _ = io::stdin().read(&mut buffer);
}
