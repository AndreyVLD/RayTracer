use crate::vector3::Vector3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.powf(1.0 / 2.2);
    }
    0.0
}

pub fn reflect(v: Vector3, normal: Vector3) -> Vector3 {
    v - 2.0 * v.dot(&normal) * normal
}

pub fn refract(v: Vector3, normal: Vector3, refractive_ratio: f64) -> Vector3 {
    let cos_theta = (-v).dot(&normal).min(1.0);
    let r_out_perp = refractive_ratio * (v + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.length().powi(2)).abs().sqrt() * normal;
    r_out_perp + r_out_parallel
}
