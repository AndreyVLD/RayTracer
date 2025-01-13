use crate::ray::Ray;
use crate::shapes::common::{HitRecord, Hittable, Surface};
use crate::vector3::Vector3;

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub surface: Surface,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
            surface: Surface::default(),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
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

        let solution: f64;

        if first_root < 0.0 && second_root < 0.0 {
            return None;
        } else if first_root < 0.0 && second_root >= 0.0 {
            solution = second_root;
        } else if first_root >= 0.0 && second_root >= 0.0 {
            solution = first_root;
        } else {
            return None;
        }

        let normal = (ray.point_at(solution) - self.center).normalize();

        Some(HitRecord::new(solution, self.surface, normal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sphere_intersection_miss_1() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 1.0);

        assert_eq!(sphere.hit(&ray), None);
    }
    #[test]
    fn test_sphere_intersection_miss_2() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, -7.0), Vector3::new(0.0, 0.0, -1.0));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 1.0);

        assert_eq!(sphere.hit(&ray), None);
    }

    #[test]
    fn test_sphere_intersection_hit() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 1.0);

        assert_eq!(
            sphere.hit(&ray),
            Some(HitRecord {
                t: 4.0,
                surface: Surface::default(),
                normal: Vector3::new(0.0, 0.0, 1.0),
            })
        );
    }

    #[test]
    fn test_sphere_intersection_tangent() {
        let ray = Ray::new(Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 1.0);

        assert_eq!(
            sphere.hit(&ray),
            Some(HitRecord {
                t: 5.0,
                surface: Surface::default(),
                normal: Vector3::new(1.0, 0.0, 0.0)
            })
        );
    }

    #[test]
    fn test_sphere_intersection_inside() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, -4.0), Vector3::new(0.0, 0.0, -1.0));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 5.0);

        assert_eq!(
            sphere.hit(&ray),
            Some(HitRecord {
                t: 6.0,
                surface: Surface::default(),
                normal: Vector3::new(0.0, 0.0, -1.0)
            })
        );
    }
}
