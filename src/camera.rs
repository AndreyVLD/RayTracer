use crate::ray::Ray;
use crate::shapes::{HitRecord, Hittable};
use crate::vector3::Vector3;

pub struct Camera {
    camera_center: Vector3,
    image_width: u32,
    image_height: u32,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    pixel00_loc: Vector3,
}

impl Camera {
    pub fn new(image_width: u32, aspect_ratio: f64) -> Camera {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        }
        let camera_center = Vector3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;

        let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let viewport_upper_left = camera_center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            camera_center,
            image_width,
            image_height,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
        }
    }

    fn get_pixel_center(&self, x: u32, y: u32) -> Vector3 {
        self.pixel00_loc + (x * self.pixel_delta_u) + (y * self.pixel_delta_v)
    }

    fn ray_color(ray: &Ray, hittable: &[Box<dyn Hittable>]) -> Vector3 {
        let a = 0.5 * (ray.direction.y + 1.0);
        let background_color =
            (1.0 - a) * Vector3::new(255.0, 255.0, 255.0) + a * Vector3::new(120.0, 200.0, 255.0);

        let mut min_ray_t = f64::INFINITY;
        let mut min_record: Option<HitRecord> = None;

        hittable.iter().for_each(|hittable| {
            if let Some(hit_record) = hittable.hit(ray) {
                if hit_record.t < min_ray_t {
                    min_ray_t = hit_record.t;
                    min_record = Some(hit_record);
                }
            }
        });
        if let Some(record) = min_record {
            (record.normal + Vector3::new(1.0, 1.0, 1.0)) * 122.0
        } else {
            background_color
        }
    }

    pub fn render(&self, hittable: Vec<Box<dyn Hittable>>) {
        let mut imgbuf = image::ImageBuffer::new(self.image_width, self.image_height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let pixel_center = self.get_pixel_center(x, y);
            let ray_direction = pixel_center - self.camera_center;

            let ray = Ray::new(self.camera_center, ray_direction);

            *pixel = Self::ray_color(&ray, &hittable).to_rgb();
        }

        if let Err(e) = imgbuf.save("output.png") {
            eprintln!("Failed to save image: {}", e);
        }
    }
}
