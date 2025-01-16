use crate::ray::Ray;
use crate::shapes::material::Material;
use crate::vector3::Vector3;
#[derive(Debug)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub poz: Vector3,
    pub normal: Vector3,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f64, poz: Vector3, normal: Vector3, material: &'a dyn Material) -> Self {
        HitRecord {
            t,
            poz,
            material,
            normal,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: (f64, f64)) -> Option<HitRecord>;
}
