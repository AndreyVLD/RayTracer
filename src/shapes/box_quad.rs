use crate::hit::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::shapes::quad::Quad;
use crate::vector3::Vector3;
use std::cmp::Ordering;
use std::sync::Arc;

/// Represents a box composed of six quads
pub struct BoxQuad {
    /// The six sides of the box, each represented as a `Hittable` quad.
    sides: Vec<Box<dyn Hittable>>,
}

impl BoxQuad {
    /// Creates a new `BoxQuad` from two opposite corners and a material.
    ///
    /// # Arguments
    ///
    /// * `a` - One corner of the box.
    /// * `b` - The opposite corner of the box.
    /// * `material` - The material to be applied to all sides of the box.
    ///
    /// # Returns
    ///
    /// A new `BoxQuad` instance.
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
    /// Checks if a ray hits any of the sides of the box within a given interval.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to test for intersection.
    /// * `interval` - The range of distances to consider for intersections.
    ///
    /// # Returns
    ///
    /// An `Option` containing the closest `HitRecord` if an intersection is found, or `None` if no intersection is found.
    fn hit(&self, ray: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        self.sides
            .iter()
            .filter_map(|s| s.hit(ray, interval))
            .min_by(|r1, r2| r1.t.partial_cmp(&r2.t).unwrap_or(Ordering::Equal))
    }
}
