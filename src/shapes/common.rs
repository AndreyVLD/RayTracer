use crate::ray::Ray;
use crate::vector3::Vector3;
#[derive(Debug, PartialEq)]
pub struct HitRecord {
    pub t: f64,
    pub poz: Vector3,
    pub surface: Surface,
    pub normal: Vector3,
}

impl HitRecord {
    pub fn new(t: f64, poz: Vector3, surface: Surface, normal: Vector3) -> HitRecord {
        HitRecord {
            t,
            poz,
            surface,
            normal,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: (f64, f64)) -> Option<HitRecord>;
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Surface {
    pub albedo: Vector3,
}

impl Surface {
    pub fn new(albedo: Vector3) -> Surface {
        Surface { albedo }
    }
}
