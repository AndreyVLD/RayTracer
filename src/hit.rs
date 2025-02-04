use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::Vector3;
#[derive(Debug)]
/// Represents a record of a hit point in the scene.
pub struct HitRecord<'a> {
    /// The parameter `t` at which the ray intersects the object.
    pub t: f64,
    /// The position of the hit point.
    pub poz: Vector3,
    /// The normal vector at the hit point.
    pub normal: Vector3,
    /// Indicates whether the hit point is on the front face of the object.
    pub front_face: bool,
    /// The material of the object at the hit point.
    pub material: &'a dyn Material,
    /// The u-coordinate for texture mapping
    pub u: f64,
    /// The v-coordinate for texture mapping.
    pub v: f64,
}

impl<'a> HitRecord<'a> {
    /// Creates a new `HitRecord` instance.
    ///
    /// # Arguments
    ///
    /// * `t` - The parameter `t` at which the ray intersects the object.
    /// * `poz` - The position of the hit point.
    /// * `material` - The material of the object at the hit point.
    /// * `u` - The u-coordinate for texture mapping.
    /// * `v` - The v-coordinate for texture mapping.
    ///
    /// # Returns
    ///
    /// A new `HitRecord` instance.
    pub fn new(t: f64, poz: Vector3, material: &'a dyn Material, u: f64, v: f64) -> Self {
        HitRecord {
            t,
            poz,
            front_face: true,
            normal: Vector3::new(1.0, 0.0, 0.0),
            material,
            u,
            v,
        }
    }

    /// Sets the face normal of the hit record based on the ray and outward normal.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray that hit the object.
    /// * `outward_normal` - The normal vector pointing outward from the hit point.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        self.front_face = ray.direction.dot(outward_normal) <= 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

/// A trait for objects that can be hit by rays.
pub trait Hittable: Send + Sync {
    /// Checks if a ray hits the object within a given interval.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to test for intersection.
    /// * `interval` - The range of distances to consider for intersections.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `HitRecord` if an intersection is found, or `None` if no intersection is found.
    fn hit(&self, ray: &Ray, interval: (f64, f64)) -> Option<HitRecord>;
}
