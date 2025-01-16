use crate::ray::Ray;
use crate::shapes::HitRecord;
use crate::utils::reflect;
use crate::vector3::Vector3;
use std::fmt::Debug;

pub trait Material: Send + Sync + Debug {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)>;
}

#[derive(Debug, Default)]
pub struct Lambertian {
    albedo: Vector3,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        let mut scatter_direction = hit_record.normal + Vector3::random_in_unit_sphere();

        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.poz, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Lambertian {
        Lambertian { albedo }
    }
}

#[derive(Debug, Default)]
pub struct Metal {
    albedo: Vector3,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        let mut reflected = reflect(ray.direction, hit_record.normal);
        reflected = reflected.normalize() + self.fuzz * Vector3::random_in_unit_sphere();

        let scattered = Ray::new(hit_record.poz, reflected);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}

impl Metal {
    pub fn new(albedo: Vector3, mut fuzz: f64) -> Metal {
        if fuzz > 1.0 {
            fuzz = 1.0
        }
        Metal { albedo, fuzz }
    }
}
