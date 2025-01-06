use crate::ray::Ray;
use crate::vector3::Vector3;

#[derive(Debug, PartialEq)]
pub struct HitRecord {
    pub t: f64,
    pub surface: Surface,
}

impl HitRecord {
    fn new(t: f64, surface: Surface) -> HitRecord {
        HitRecord { t, surface }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Surface {
    pub albedo: Vector3,
}

impl Surface {
    pub fn new(albedo: Vector3) -> Surface {
        Surface { albedo }
    }
}

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
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&(ray.origin - self.center));
        let c =
            (ray.origin - self.center).dot(&(ray.origin - self.center)) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let first_root = (-b - sqrt_d) / (2.0 * a);
        let second_root = (-b + sqrt_d) / (2.0 * a);

        if first_root < 0.0 && second_root < 0.0 {
            None
        } else if first_root < 0.0 && second_root >= 0.0 {
            Some(HitRecord::new(second_root, self.surface))
        } else if first_root >= 0.0 && second_root >= 0.0 {
            Some(HitRecord::new(first_root, self.surface))
        } else {
            None
        }
    }
}

pub struct Quad {
    pub bottom_left: Vector3,
    pub top_left: Vector3,
    pub top_right: Vector3,
    pub bottom_right: Vector3,
}

impl Quad {
    pub fn new(
        bottom_left: Vector3,
        top_left: Vector3,
        top_right: Vector3,
        bottom_right: Vector3,
    ) -> Quad {
        Quad {
            bottom_right,
            top_left,
            top_right,
            bottom_left,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {}
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
                surface: Surface::default()
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
                surface: Surface::default()
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
                surface: Surface::default()
            })
        );
    }
}
