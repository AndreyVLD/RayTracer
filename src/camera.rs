#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

use crate::hit::Hittable;
use crate::ray::Ray;
use crate::utils::linear_to_gamma;
use crate::vector3::Vector3;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

pub struct Camera {
    /// The aspect ratio of the camera.
    aspect_ratio: f64,
    /// The width of the image in pixels.
    image_width: u32,
    /// The number of samples per pixel for Anti-Aliasing.
    samples_per_pixel: u32,
    /// The maximum depth for ray tracing.
    max_depth: u32,
    /// The background function that returns a color for a given direction.
    background: fn(Vector3) -> Vector3,

    /// The center of the camera.
    camera_center: Vector3,
    /// The height of the image in pixels.
    image_height: u32,
    /// The change in position per pixel in the u direction.
    pixel_delta_u: Vector3,
    /// The change in position per pixel in the v direction.
    pixel_delta_v: Vector3,
    /// The location of the top-left pixel.
    pixel00_loc: Vector3,
    /// The angle of defocus
    defocus_angle: f64,
    /// The u component of the defocus disk.
    defocus_disk_u: Vector3,
    /// The v component of the defocus disk.
    defocus_disk_v: Vector3,
}

impl Camera {
    /// Creates a new `Camera` instance.
    ///
    /// # Arguments
    ///
    /// * `image_width` - The width of the image in pixels.
    /// * `aspect_ratio` - The aspect ratio of the camera.
    /// * `samples_per_pixel` - The number of samples per pixel.
    /// * `max_depth` - The maximum depth for ray tracing.
    /// * `background` - The background function that returns a color for a given direction.
    /// * `vfov` - The vertical field of view in degrees.
    /// * `look_from` - The position of the camera.
    /// * `look_at` - The point the camera is looking at.
    /// * `vup` - The up direction of the camera.
    /// * `defocus_angle` - The angle of defocus.
    /// * `focus_dist` - The distance to the focus plane.
    ///
    /// # Returns
    ///
    /// A new `Camera` instance.
    pub fn new(
        image_width: u32,
        aspect_ratio: f64,
        samples_per_pixel: u32,
        max_depth: u32,
        background: fn(Vector3) -> Vector3,
        vfov: f64,
        look_from: Vector3,
        look_at: Vector3,
        vup: Vector3,
        defocus_angle: f64,
        mut focus_dist: f64,
    ) -> Camera {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        }
        let camera_center = look_from;

        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        if focus_dist <= 0.0 {
            focus_dist = 1.0;
        }

        let viewport_height = 2.0 * h * focus_dist;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let viewport_upper_left =
            camera_center - focus_dist * w - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            image_height,
            max_depth,
            background,

            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    /// Returns the center of the pixel at the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the pixel.
    /// * `y` - The y-coordinate of the pixel.
    ///
    /// # Returns
    ///
    /// The center of the pixel as a `Vector3`.
    fn get_pixel_center(&self, x: u32, y: u32) -> Vector3 {
        self.pixel00_loc + (x * self.pixel_delta_u) + (y * self.pixel_delta_v)
    }

    /// Returns a ray that passes through the pixel at the given coordinates.
    /// The ray is randomly offset within the pixel to provide Anti-Aliasing.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the pixel.
    /// * `y` - The y-coordinate of the pixel.
    ///
    /// # Returns
    ///
    /// A `Ray` that passes through the pixel.
    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset_x = fastrand::f64() - 0.5;
        let offset_y = fastrand::f64() - 0.5;

        let pixel_sample = self.pixel00_loc
            + ((x as f64 + offset_x) * self.pixel_delta_u)
            + ((y as f64 + offset_y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    /// Returns a random sample point on the defocus disk.
    ///
    /// # Returns
    ///
    /// A random sample point on the defocus disk as a `Vector3`.
    fn defocus_disk_sample(&self) -> Vector3 {
        let p = Vector3::random_in_unit_disk();
        self.camera_center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    /// Computes the color of a ray by tracing it through the scene.
    /// Main ray tracing function. Recursively traces rays through the scene.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to trace.
    /// * `hittable` - The list of objects in the scene.
    /// * `depth` - The current depth of the ray.
    ///
    /// # Returns
    ///
    /// The color of the ray as a `Vector3`.
    fn ray_color(&self, ray: &Ray, hittable: &[Box<dyn Hittable>], depth: u32) -> Vector3 {
        if depth == 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        let min_record = hittable
            .iter()
            .filter_map(|hittable| hittable.hit(ray, (0.001, f64::INFINITY)))
            .min_by(|r1, r2| r1.t.partial_cmp(&r2.t).unwrap_or(Ordering::Equal));

        if let Some(record) = min_record {
            let emission_color = record.material.emitted(record.u, record.v, &record.poz);

            if let Some((scattered, attenuation)) = record.material.scatter(ray, &record) {
                let scatter_color = attenuation * self.ray_color(&scattered, hittable, depth - 1);
                scatter_color + emission_color
            } else {
                emission_color
            }
        } else {
            (self.background)(ray.direction)
        }
    }

    /// Renders the scene and saves the image to a file.
    ///
    /// # Arguments
    ///
    /// * `hittable` - The list of objects in the scene.
    pub fn render(&self, hittable: Vec<Box<dyn Hittable>>) {
        let progress = Arc::new(AtomicUsize::new(10));
        let total_pixels = (self.image_width * self.image_height) as usize;

        println!("Rendering...");

        let mut imgbuf = image::ImageBuffer::new(self.image_width, self.image_height);
        imgbuf
            .enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(x, y, pixel)| {
                let mut initial_color = Vector3::default();

                for _s in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    let color = self.ray_color(&ray, &hittable, self.max_depth);
                    initial_color += color;
                }
                initial_color = initial_color / self.samples_per_pixel as f64;

                // Apply a linear to gamma transform for gamma 2, clamping and conversion to bytes
                initial_color = Vector3::new(
                    255.0 * linear_to_gamma(initial_color.x).clamp(0.0, 1.0),
                    255.0 * linear_to_gamma(initial_color.y).clamp(0.0, 1.0),
                    255.0 * linear_to_gamma(initial_color.z).clamp(0.0, 1.0),
                );

                *pixel = initial_color.to_rgb();

                let current_progress = progress.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                if current_progress % (total_pixels / 10) == 0 {
                    println!("Progress: {}%", (current_progress * 100) / total_pixels);
                }
            });

        let output_name = "output.png";
        if let Err(e) = imgbuf.save(output_name) {
            eprintln!("Failed to save image: {}", e);
        } else {
            println!("Successfully saved image to {}", output_name);
        }
    }
}
