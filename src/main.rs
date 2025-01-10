use RayTracerRust::generate_image;

fn main() {
    const WIDTH: u32 = 400;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    generate_image(WIDTH, ASPECT_RATIO);
}
