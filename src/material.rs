use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidTexture, Texture};
use crate::utils::{reflect, refract};
use crate::vector3::Vector3;
use std::fmt::Debug;

pub trait Material: Send + Sync + Debug {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)>;

    fn emitted(&self, _u: f64, _v: f64, _p: &Vector3) -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

#[derive(Debug)]
pub struct Lambertian {
    texture: Box<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        let mut scatter_direction = hit_record.normal + Vector3::random_in_unit_sphere();

        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.poz, scatter_direction);
        let attenuation = self
            .texture
            .value(hit_record.u, hit_record.v, &hit_record.poz);
        Some((scattered, attenuation))
    }
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Lambertian {
        Lambertian {
            texture: Box::new(SolidTexture::new(albedo)),
        }
    }

    pub fn from_texture(texture: Box<dyn Texture>) -> Lambertian {
        Lambertian { texture }
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

#[derive(Debug, Default)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let cos_theta = (-ray.direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || Self::reflectance(cos_theta, self.refraction_index) > fastrand::f64()
        {
            reflect(ray.direction, hit_record.normal)
        } else {
            refract(ray.direction, hit_record.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit_record.poz, direction);
        Some((scattered, attenuation))
    }
}

#[derive(Debug)]
pub struct DiffuseLight {
    texture: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Vector3) -> DiffuseLight {
        DiffuseLight {
            texture: Box::new(SolidTexture::new(emit)),
        }
    }

    pub fn from_texture(texture: Box<dyn Texture>) -> DiffuseLight {
        DiffuseLight { texture }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Vector3) -> Vector3 {
        self.texture.value(u, v, p)
    }
}

#[derive(Debug)]
pub struct Isotropic {
    texture: Box<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Vector3) -> Isotropic {
        Isotropic {
            texture: Box::new(SolidTexture::new(albedo)),
        }
    }

    pub fn from_texture(texture: Box<dyn Texture>) -> Isotropic {
        Isotropic { texture }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        let scattered = Ray::new(hit_record.poz, Vector3::random_in_unit_sphere());

        let attenuation = self
            .texture
            .value(hit_record.u, hit_record.v, &hit_record.poz);
        Some((scattered, attenuation))
    }
}
