use crate::vector3::Vector3;

/// Converts a linear color component to a gamma-corrected component.
///
/// # Arguments
///
/// * `linear_component` - The linear color component.
///
/// # Returns
///
/// The gamma-corrected color component.
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.powf(1.0 / 2.2);
    }
    0.0
}

/// Reflects a vector off a surface with a given normal.
///
/// # Arguments
///
/// * `v` - The incoming vector.
/// * `normal` - The normal vector of the surface.
///
/// # Returns
///
/// The reflected vector.
pub fn reflect(v: Vector3, normal: Vector3) -> Vector3 {
    v - 2.0 * v.dot(&normal) * normal
}

/// Refracts a vector through a surface with a given normal and refractive index ratio.
///
/// # Arguments
///
/// * `v` - The incoming vector.
/// * `normal` - The normal vector of the surface.
/// * `refractive_ratio` - The ratio of the refractive indices.
///
/// # Returns
///
/// The refracted vector.
pub fn refract(v: Vector3, normal: Vector3, refractive_ratio: f64) -> Vector3 {
    let cos_theta = (-v).dot(&normal).min(1.0);
    let r_out_perp = refractive_ratio * (v + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.length().powi(2)).abs().sqrt() * normal;
    r_out_perp + r_out_parallel
}

/// Generates a background gradient color based on the input vector.
///
/// # Arguments
///
/// * `v` - The input vector.
///
/// # Returns
///
/// The gradient color as a `Vector3`.
pub fn background_gradient(v: Vector3) -> Vector3 {
    let a = 0.5 * (v.y + 1.0);
    (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0)
}
