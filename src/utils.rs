use crate::vector3::Vector3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

pub fn reflect(v: Vector3, normal: Vector3) -> Vector3 {
    v - 2.0 * v.dot(&normal) * normal
}
