use crate::ray::Ray;
use crate::vector3::Vector3;

#[derive(Debug, PartialEq)]
pub struct HitRecord {
    pub t: f64,
    pub surface: Surface,
    pub normal: Vector3,
}

impl HitRecord {
    pub(crate) fn new(t: f64, surface: Surface, normal: Vector3) -> HitRecord {
        HitRecord { t, surface, normal }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
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
