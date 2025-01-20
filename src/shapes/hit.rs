use crate::ray::Ray;
use crate::shapes::material::Material;
use crate::vector3::Vector3;
#[derive(Debug)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub poz: Vector3,
    pub normal: Vector3,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f64, poz: Vector3, material: &'a dyn Material) -> Self {
        HitRecord {
            t,
            poz,
            front_face: false,
            normal: Vector3::default(),
            material,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        self.front_face = ray.direction.dot(outward_normal) <= 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: (f64, f64)) -> Option<HitRecord>;
}
