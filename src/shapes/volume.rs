use crate::hit::{HitRecord, Hittable};
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vector3::Vector3;
use fastrand::f64;
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    neg_inv_density: f64,
    material: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hittable>, density: f64, color: Vector3) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            material: Arc::new(Isotropic::new(color)),
        }
    }

    pub fn from_texture(
        boundary: Box<dyn Hittable>,
        density: f64,
        texture: Box<dyn Texture>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            material: Arc::new(Isotropic::from_texture(texture)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        if let Some(mut hit1) = self.boundary.hit(ray, (f64::NEG_INFINITY, f64::INFINITY)) {
            return if let Some(mut hit2) = self.boundary.hit(ray, (hit1.t + 0.0001, f64::INFINITY))
            {
                if hit1.t < interval.0 {
                    hit1.t = interval.0;
                }

                if hit2.t > interval.1 {
                    hit2.t = interval.1;
                }

                if hit1.t >= hit2.t {
                    return None;
                }

                if hit1.t < 0.0 {
                    hit1.t = 0.0;
                }

                let distance_inside_boundary = (hit2.t - hit1.t) * ray.length;
                let hit_distance = self.neg_inv_density * f64().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let t = hit1.t + hit_distance / ray.length;
                let hit_record = HitRecord::new(t, ray.point_at(t), &*self.material, 0.0, 0.0);
                Some(hit_record)
            } else {
                None
            };
        }
        None
    }
}
