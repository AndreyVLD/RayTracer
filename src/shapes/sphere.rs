use crate::ray::Ray;
use crate::shapes::hit::{HitRecord, Hittable};
use crate::shapes::material::Material;
use crate::vector3::Vector3;

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
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

        let solution = if first_root >= 0.0 {
            first_root
        } else if second_root >= 0.0 {
            second_root
        } else {
            return None;
        };

        if !(solution >= interval.0 && solution <= interval.1) {
            return None;
        }

        let normal = (ray.point_at(solution) - self.center).normalize();

        Some(HitRecord::new(
            solution,
            ray.point_at(solution),
            normal,
            &*self.material,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::Lambertian;

    #[test]
    fn test_sphere_intersection_miss_1() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let material = Box::new(Lambertian::new(Vector3::new(1.0, 1.0, 1.0)));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 1.0, material);

        assert!(sphere.hit(&ray, (-10.0, 10.0)).is_none());
    }
    #[test]
    fn test_sphere_intersection_miss_2() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, -7.0), Vector3::new(0.0, 0.0, -1.0));
        let material = Box::new(Lambertian::new(Vector3::new(1.0, 1.0, 1.0)));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 1.0, material);

        assert!(sphere.hit(&ray, (-10.0, 10.0)).is_none());
    }

    #[test]
    fn test_sphere_intersection_hit() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0));
        let material = Box::new(Lambertian::new(Vector3::new(1.0, 1.0, 1.0)));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 1.0, material);
        let hit_record = sphere.hit(&ray, (-10.0, 10.0)).unwrap();

        assert_eq!(hit_record.t, 4.0);
        assert_eq!(hit_record.poz, Vector3::new(0.0, 0.0, -4.0));
        assert_eq!(hit_record.normal, Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_sphere_intersection_tangent() {
        let ray = Ray::new(Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0));
        let material = Box::new(Lambertian::new(Vector3::new(1.0, 1.0, 1.0)));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 1.0, material);
        let hit_record = sphere.hit(&ray, (-10.0, 10.0)).unwrap();

        assert_eq!(hit_record.t, 5.0);
        assert_eq!(hit_record.poz, Vector3::new(1.0, 0.0, -5.0));
        assert_eq!(hit_record.normal, Vector3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_sphere_intersection_inside() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, -4.0), Vector3::new(0.0, 0.0, -1.0));
        let material = Box::new(Lambertian::new(Vector3::new(1.0, 1.0, 1.0)));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 5.0, material);
        let hit_record = sphere.hit(&ray, (-10.0, 10.0)).unwrap();

        assert_eq!(hit_record.t, 6.0);
        assert_eq!(hit_record.poz, Vector3::new(0.0, 0.0, -10.0));
        assert_eq!(hit_record.normal, Vector3::new(0.0, 0.0, -1.0));
    }
}
