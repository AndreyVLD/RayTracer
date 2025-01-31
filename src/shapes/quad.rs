use crate::hit::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::Vector3;
use std::sync::Arc;

/// Represents a quadrilateral shape in 3D space.
pub struct Quad {
    /// The starting corner of the quad.
    starting_corner: Vector3,
    /// The vector representing one edge of the quad.
    u: Vector3,
    /// The vector representing the adjacent edge of the quad.
    v: Vector3,
    /// The material of the quad
    material: Arc<dyn Material>,
    /// The normal vector of the quad.
    normal: Vector3,
    /// The distance from the origin to the plane of the quad.
    d: f64,
    /// The vector used for intersection calculations.
    w: Vector3,
}

impl Quad {
    /// Creates a new `Quad` from a starting corner, two edge vectors, and a material.
    ///
    /// # Arguments
    ///
    /// * `starting_corner` - The starting corner of the quad.
    /// * `u` - The vector representing one edge of the quad.
    /// * `v` - The vector representing the adjacent edge of the quad.
    /// * `material` - The material of the quad.
    ///
    /// # Returns
    ///
    /// A new `Quad` instance.
    pub fn new(
        starting_corner: Vector3,
        u: Vector3,
        v: Vector3,
        material: Arc<dyn Material>,
    ) -> Quad {
        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&starting_corner);
        let w = n / n.dot(&n);

        Quad {
            starting_corner,
            u,
            v,
            material,
            normal,
            d,
            w,
        }
    }
}

impl Hittable for Quad {
    /// Checks if a ray hits the quad within a given interval.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to test for intersection.
    /// * `interval` - The range of distances to consider for intersections.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `HitRecord` if an intersection is found, or `None` if no intersection is found.
    fn hit(&self, ray: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);

        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(&ray.origin)) / denom;

        if !(t >= interval.0 && t <= interval.1) {
            return None;
        }

        let intersection = ray.point_at(t);

        let planar_hit_point_intersection = intersection - self.starting_corner;
        let alpha = self.w.dot(&planar_hit_point_intersection.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hit_point_intersection));

        if alpha > 1.0 || beta > 1.0 || alpha < 0.0 || beta < 0.0 {
            return None;
        }

        let mut record = HitRecord::new(t, intersection, &*self.material, alpha, beta);
        record.set_face_normal(ray, &self.normal);
        Some(record)
    }
}
