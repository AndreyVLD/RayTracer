use crate::hit::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vector3::Vector3;
use std::sync::Arc;

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vector3,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vector3) -> Self {
        Self { object, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        let ray_offset = Ray::new(ray.origin - self.offset, ray.direction);

        if let Some(mut hit_record) = self.object.hit(&ray_offset, interval) {
            hit_record.poz += self.offset;
            Some(hit_record)
        } else {
            None
        }
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        RotateY {
            object,
            cos_theta,
            sin_theta,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        let origin = Vector3::new(
            (self.cos_theta * ray.origin.x) - (self.sin_theta * ray.origin.z),
            ray.origin.y,
            (self.sin_theta * ray.origin.x) + (self.cos_theta * ray.origin.z),
        );

        let direction = Vector3::new(
            (self.cos_theta * ray.direction.x) - (self.sin_theta * ray.direction.z),
            ray.direction.y,
            (self.sin_theta * ray.direction.x) + (self.cos_theta * ray.direction.z),
        );

        let rotated_ray = Ray::new(origin, direction);

        if let Some(mut hit_record) = self.object.hit(&rotated_ray, interval) {
            hit_record.poz = Vector3::new(
                (self.cos_theta * hit_record.poz.x) + (self.sin_theta * hit_record.poz.z),
                hit_record.poz.y,
                (-self.sin_theta * hit_record.poz.x) + (self.cos_theta * hit_record.poz.z),
            );

            hit_record.normal = Vector3::new(
                (self.cos_theta * hit_record.normal.x) + (self.sin_theta * hit_record.normal.z),
                hit_record.normal.y,
                (-self.sin_theta * hit_record.normal.x) + (self.cos_theta * hit_record.normal.z),
            );

            Some(hit_record)
        } else {
            None
        }
    }
}
