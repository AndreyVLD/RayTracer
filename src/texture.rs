use crate::vector3::Vector3;
use std::fmt::Debug;

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
