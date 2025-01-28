use crate::ray::Ray;
use crate::shapes::quad::Quad;
use crate::shapes::{HitRecord, Hittable, Material};
use crate::vector3::Vector3;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BoxQuad {
    sides: Vec<Box<dyn Hittable>>,
}

impl BoxQuad {
    pub fn new(a: Vector3, b: Vector3, material: Arc<dyn Material>) -> Self {
        let mut sides: Vec<Box<dyn Hittable>> = Vec::new();

        let min = Vector3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Vector3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        let dx = Vector3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vector3::new(0.0, max.y - min.y, 0.0);
        let dz = Vector3::new(0.0, 0.0, max.z - min.z);

        sides.push(Box::new(Quad::new(
            Vector3::new(min.x, min.y, max.z),
            dx,
            dy,
            material.clone(),
        ))); // front

        sides.push(Box::new(Quad::new(
            Vector3::new(max.x, min.y, max.z),
            -dz,
            dy,
            material.clone(),
        ))); // right

        sides.push(Box::new(Quad::new(
            Vector3::new(max.x, min.y, min.z),
            -dx,
            dy,
            material.clone(),
        ))); // back

        sides.push(Box::new(Quad::new(
            Vector3::new(min.x, min.y, min.z),
            dz,
            dy,
            material.clone(),
        ))); // left

        sides.push(Box::new(Quad::new(
            Vector3::new(min.x, max.y, max.z),
            dx,
            -dz,
            material.clone(),
        ))); // top

        sides.push(Box::new(Quad::new(
            Vector3::new(min.x, min.y, min.z),
            dx,
            dz,
            material.clone(),
        ))); // bottom
        Self { sides }
    }
}

impl Hittable for BoxQuad {
    fn hit(&self, ray: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        self.sides
            .iter()
            .filter_map(|s| s.hit(ray, interval))
            .min_by(|r1, r2| r1.t.partial_cmp(&r2.t).unwrap_or(Ordering::Equal))
    }
}
