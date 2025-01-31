use crate::hit::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::Vector3;
use std::sync::Arc;

/// Represents a sphere in 3D space.
pub struct Sphere {
    /// The center of the sphere.
    center: Vector3,
    /// The radius of the sphere.
    radius: f64,
    /// The material of the sphere.
    material: Arc<dyn Material>,
}

impl Sphere {
    /// Creates a new `Sphere` with the given center, radius, and material.
    ///
    /// # Arguments
    ///
    /// * `center` - The center of the sphere.
    /// * `radius` - The radius of the sphere.
    /// * `material` - The material of the sphere.
    ///
    /// # Returns
    ///
    /// A new `Sphere` instance.
    pub fn new(center: Vector3, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    /// Computes the spherical coordinates (u, v) for a given point on the sphere.
    ///
    /// # Arguments
    ///
    /// * `p` - The point on the sphere.
    ///
    /// # Returns
    ///
    /// A tuple containing the spherical coordinates (u, v).
    fn get_sphere_uv(p: Vector3) -> (f64, f64) {
        let phi = (-p.z).atan2(p.x) + std::f64::consts::PI;
        let theta = (-p.y).acos();

        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    /// Checks if a ray hits the sphere within a given interval.
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
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let first_root = (-b - sqrt_d) / (2.0 * a);
        let second_root = (-b + sqrt_d) / (2.0 * a);

        let solution = if first_root > interval.0 {
            first_root
        } else if second_root > interval.0 {
            second_root
        } else {
            return None;
        };

        if solution > interval.1 {
            return None;
        }

        let outward_normal = (ray.point_at(solution) - self.center).normalize();
        let (u, v) = Sphere::get_sphere_uv(outward_normal);
        let mut hit = HitRecord::new(solution, ray.point_at(solution), &*self.material, u, v);
        hit.set_face_normal(ray, &outward_normal);

        Some(hit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Lambertian;

    #[test]
    fn test_sphere_intersection_miss_1() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let material = Arc::new(Lambertian::new(Vector3::new(1.0, 1.0, 1.0)));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 1.0, material);

        assert!(sphere.hit(&ray, (-10.0, 10.0)).is_none());
    }
    #[test]
    fn test_sphere_intersection_miss_2() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, -7.0), Vector3::new(0.0, 0.0, -1.0));
        let material = Arc::new(Lambertian::new(Vector3::new(1.0, 1.0, 1.0)));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 1.0, material);

        assert!(sphere.hit(&ray, (-10.0, 10.0)).is_none());
    }

    #[test]
    fn test_sphere_intersection_hit() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0));
        let material = Arc::new(Lambertian::new(Vector3::new(1.0, 1.0, 1.0)));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 1.0, material);
        let hit_record = sphere.hit(&ray, (-10.0, 10.0)).unwrap();

        assert_eq!(hit_record.t, 4.0);
        assert_eq!(hit_record.poz, Vector3::new(0.0, 0.0, -4.0));
        assert_eq!(hit_record.normal, Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_sphere_intersection_tangent() {
        let ray = Ray::new(Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0));
        let material = Arc::new(Lambertian::new(Vector3::new(1.0, 1.0, 1.0)));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 1.0, material);
        let hit_record = sphere.hit(&ray, (-10.0, 10.0)).unwrap();

        assert_eq!(hit_record.t, 5.0);
        assert_eq!(hit_record.poz, Vector3::new(1.0, 0.0, -5.0));
        assert_eq!(hit_record.normal, Vector3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_sphere_intersection_inside() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, -4.0), Vector3::new(0.0, 0.0, -1.0));
        let material = Arc::new(Lambertian::new(Vector3::new(1.0, 1.0, 1.0)));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 5.0, material);
        let hit_record = sphere.hit(&ray, (-10.0, 10.0)).unwrap();

        assert_eq!(hit_record.t, 6.0);
        assert_eq!(hit_record.poz, Vector3::new(0.0, 0.0, -10.0));
        assert_eq!(hit_record.normal, Vector3::new(-0.0, -0.0, 1.0));
    }
}
