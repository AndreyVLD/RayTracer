use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidTexture, Texture};
use crate::utils::{reflect, refract};
use crate::vector3::Vector3;
use std::fmt::Debug;

/// A trait for materials that can scatter rays and emit light
pub trait Material: Send + Sync + Debug {
    /// Scatters a ray upon hitting the material.
    ///
    /// # Arguments
    ///
    /// * `ray` - The incoming ray.
    /// * `hit_record` - The record of the hit point.
    ///
    /// # Returns
    ///
    /// An `Option` containing a tuple of the scattered ray and the attenuation vector, or `None` if no scattering occurs.
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)>;

    /// Returns the emitted light from the material at a given point.
    ///
    /// # Arguments
    ///
    /// * `_u` - The u-coordinate for texture mapping.
    /// * `_v` - The v-coordinate for texture mapping.
    /// * `_p` - The position at which the light is emitted.
    ///
    /// # Returns
    ///
    /// The emitted light as a `Vector3`.
    fn emitted(&self, _u: f64, _v: f64, _p: &Vector3) -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

/// Represents a Lambertian (diffuse) material.
#[derive(Debug)]
pub struct Lambertian {
    /// The texture of the material.
    texture: Box<dyn Texture>,
}

impl Material for Lambertian {
    /// Scatters a ray upon hitting the Lambertian material.
    ///
    /// # Arguments
    ///
    /// * `_ray` - The incoming ray.
    /// * `hit_record` - The record of the hit point.
    ///
    /// # Returns
    ///
    /// An `Option` containing a tuple of the scattered ray and the attenuation vector, or `None` if no scattering occurs.
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
    /// Creates a new Lambertian material with a solid color.
    ///
    /// # Arguments
    ///
    /// * `albedo` - The color of the material.
    ///
    /// # Returns
    ///
    /// A new `Lambertian` instance.
    pub fn new(albedo: Vector3) -> Lambertian {
        Lambertian {
            texture: Box::new(SolidTexture::new(albedo)),
        }
    }

    /// Creates a new Lambertian material with a texture.
    ///
    /// # Arguments
    ///
    /// * `texture` - The texture of the material.
    ///
    /// # Returns
    ///
    /// A new `Lambertian` instance.
    pub fn from_texture(texture: Box<dyn Texture>) -> Lambertian {
        Lambertian { texture }
    }
}

/// Represents a metallic material.
#[derive(Debug, Default)]
pub struct Metal {
    /// The albedo (color) of the material.
    albedo: Vector3,
    /// The fuzziness of the reflection.
    fuzz: f64,
}

impl Material for Metal {
    /// Scatters a ray upon hitting the metallic material.
    ///
    /// # Arguments
    ///
    /// * `ray` - The incoming ray.
    /// * `hit_record` - The record of the hit point.
    ///
    /// # Returns
    ///
    /// An `Option` containing a tuple of the scattered ray and the attenuation vector, or `None` if no scattering occurs
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        let mut reflected = reflect(ray.direction, hit_record.normal);
        reflected = reflected.normalize() + self.fuzz * Vector3::random_in_unit_sphere();

        let scattered = Ray::new(hit_record.poz, reflected);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}

impl Metal {
    /// Creates a new metallic material.
    ///
    /// # Arguments
    ///
    /// * `albedo` - The color of the material.
    /// * `fuzz` - The fuzziness of the reflection.
    ///
    /// # Returns
    ///
    /// A new `Metal` instance.
    pub fn new(albedo: Vector3, mut fuzz: f64) -> Metal {
        if fuzz > 1.0 {
            fuzz = 1.0
        }
        Metal { albedo, fuzz }
    }
}

/// Represents a dielectric (transparent) material.
#[derive(Debug, Default)]
pub struct Dielectric {
    /// The index of refraction of the material.
    refraction_index: f64,
}

impl Dielectric {
    /// Creates a new dielectric material.
    ///
    /// # Arguments
    ///
    /// * `refraction_index` - The index of refraction of the material.
    ///
    /// # Returns
    ///
    /// A new `Dielectric` instance.
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    /// Computes the reflectance using Schlick's approximation.
    ///
    /// # Arguments
    ///
    /// * `cosine` - The cosine of the angle of incidence.
    /// * `refraction_index` - The index of refraction.
    ///
    /// # Returns
    ///
    /// The reflectance as a `f64`.
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    /// Scatters a ray upon hitting the dielectric material.
    ///
    /// # Arguments
    ///
    /// * `ray` - The incoming ray.
    /// * `hit_record` - The record of the hit point.
    ///
    /// # Returns
    ///
    /// An `Option` containing a tuple of the scattered ray and the attenuation vector, or `None` if no scattering occurs.
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

/// Represents a diffuse light material.
#[derive(Debug)]
pub struct DiffuseLight {
    /// The texture of the light.
    texture: Box<dyn Texture>,
}

impl DiffuseLight {
    /// Creates a new diffuse light material with a solid color.
    ///
    /// # Arguments
    ///
    /// * `emit` - The color of the light.
    ///
    /// # Returns
    ///
    /// A new `DiffuseLight` instance.
    pub fn new(emit: Vector3) -> DiffuseLight {
        DiffuseLight {
            texture: Box::new(SolidTexture::new(emit)),
        }
    }

    /// Creates a new diffuse light material with a texture.
    ///
    /// # Arguments
    ///
    /// * `texture` - The texture of the light.
    ///
    /// # Returns
    ///
    /// A new `DiffuseLight` instance.
    pub fn from_texture(texture: Box<dyn Texture>) -> DiffuseLight {
        DiffuseLight { texture }
    }
}

impl Material for DiffuseLight {
    /// Diffuse light materials do not scatter rays.
    ///
    /// # Arguments
    ///
    /// * `_ray` - The incoming ray.
    /// * `_hit_record` - The record of the hit point.
    ///
    /// # Returns
    ///
    /// Always returns `None`.
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        None
    }

    /// Returns the emitted light from the diffuse light material at a given point.
    ///
    /// # Arguments
    ///
    /// * `u` - The u-coordinate for texture mapping.
    /// * `v` - The v-coordinate for texture mapping.
    /// * `p` - The position at which the light is emitted.
    ///
    /// # Returns
    ///
    /// The emitted light as a `Vector3`.
    fn emitted(&self, u: f64, v: f64, p: &Vector3) -> Vector3 {
        self.texture.value(u, v, p)
    }
}

/// Represents an isotropic (scattering in all directions) material.
#[derive(Debug)]
pub struct Isotropic {
    /// The texture of the material.
    texture: Box<dyn Texture>,
}

impl Isotropic {
    /// Creates a new isotropic material with a solid color.
    ///
    /// # Arguments
    ///
    /// * `albedo` - The color of the material.
    ///
    /// # Returns
    ///
    /// A new `Isotropic` instance.
    pub fn new(albedo: Vector3) -> Isotropic {
        Isotropic {
            texture: Box::new(SolidTexture::new(albedo)),
        }
    }

    /// Creates a new isotropic material with a texture.
    ///
    /// # Arguments
    ///
    /// * `texture` - The texture of the material.
    ///
    /// # Returns
    ///
    /// A new `Isotropic` instance.
    pub fn from_texture(texture: Box<dyn Texture>) -> Isotropic {
        Isotropic { texture }
    }
}

impl Material for Isotropic {
    // Scatters a ray upon hitting the isotropic material.
    ///
    /// # Arguments
    ///
    /// * `_ray` - The incoming ray.
    /// * `hit_record` - The record of the hit point.
    ///
    /// # Returns
    ///
    /// An `Option` containing a tuple of the scattered ray and the attenuation vector, or `None` if no scattering occurs.
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        let scattered = Ray::new(hit_record.poz, Vector3::random_in_unit_sphere());

        let attenuation = self
            .texture
            .value(hit_record.u, hit_record.v, &hit_record.poz);
        Some((scattered, attenuation))
    }
}
