#![allow(unused)]
use crate::vector3::Vector3;
use image::{DynamicImage, GenericImageView, ImageReader};
use std::fmt::Debug;
use std::path::{Path, PathBuf};

pub trait Texture: Send + Sync + Debug {
    fn value(&self, u: f64, v: f64, point: &Vector3) -> Vector3;
}

#[derive(Debug)]
pub struct SolidTexture {
    albedo: Vector3,
}

impl SolidTexture {
    pub fn new(albedo: Vector3) -> SolidTexture {
        SolidTexture { albedo }
    }
}

impl Texture for SolidTexture {
    fn value(&self, _u: f64, _v: f64, _point: &Vector3) -> Vector3 {
        self.albedo
    }
}

#[derive(Debug)]
pub struct CheckerTexture {
    scale: f64,
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, odd: Vector3, even: Vector3) -> CheckerTexture {
        CheckerTexture {
            scale,
            odd: Box::new(SolidTexture::new(odd)),
            even: Box::new(SolidTexture::new(even)),
        }
    }
    pub fn from_texture(
        scale: f64,
        odd: Box<dyn Texture>,
        even: Box<dyn Texture>,
    ) -> CheckerTexture {
        CheckerTexture { scale, odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vector3) -> Vector3 {
        let x = (self.scale * p.x).floor() as i32;
        let y = (self.scale * p.y).floor() as i32;
        let z = (self.scale * p.z).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

#[derive(Debug)]
pub struct ImageTexture {
    data: DynamicImage,
}

impl ImageTexture {
    pub fn new(file_name: &str) -> ImageTexture {
        if let Some(path) = Self::find_file(file_name) {
            let image_reader = ImageReader::open(path).expect("Failed to open image file");
            let image_data = image_reader.decode().expect("Failed to decode image");
            ImageTexture { data: image_data }
        } else {
            eprintln!("Failed to find image file");
            ImageTexture {
                data: DynamicImage::new_rgb8(0, 0),
            }
        }
    }

    fn find_file(file_name: &str) -> Option<PathBuf> {
        let paths_to_check = [
            file_name,
            &format!("./{}", file_name),
            &format!("textures/{}", file_name),
            &format!("../textures/{}", file_name),
            &format!("../../textures/{}", file_name),
            &format!("../../../textures/{}", file_name),
            &format!("../../../../textures/{}", file_name),
        ];

        paths_to_check
            .iter()
            .map(Path::new)
            .find(|path| path.exists())
            .map(Path::to_path_buf)
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, p: &Vector3) -> Vector3 {
        if self.data.height() == 0 {
            return Vector3::new(0.0, 1.0, 1.0);
        }

        u = u.clamp(0.0, 1.0);
        v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * (self.data.width() as f64)) as u32;
        let j = (v * (self.data.height() as f64)) as u32;

        let pixel = self.data.get_pixel(i, j);
        let r_srgb = pixel[0] as f64 / 255.0;
        let g_srgb = pixel[1] as f64 / 255.0;
        let b_srgb = pixel[2] as f64 / 255.0;

        // Convert texture from Gamma to Linear colors
        Vector3::new(r_srgb.powf(2.2), g_srgb.powf(2.2), b_srgb.powf(2.2))
    }
}
