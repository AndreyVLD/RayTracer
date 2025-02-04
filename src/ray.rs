use crate::vector3::Vector3;

/// Represents a ray in 3D space
pub struct Ray {
    /// The origin point of the ray.
    pub origin: Vector3,
    /// The direction vector of the ray.
    pub direction: Vector3,
    /// The length of the ray.
    pub length: f64,
}

impl Ray {
    /// Creates a new `Ray` with the given origin and direction.
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin point of the ray.
    /// * `direction` - The direction vector of the ray.
    ///
    /// # Returns
    ///
    /// A new `Ray` instance
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
            length: direction.length(),
        }
    }

    /// Computes the point at a given distance `t` along the ray.
    ///
    /// # Arguments
    ///
    /// * `t` - The distance along the ray.
    ///
    /// # Returns
    ///
    /// The point at distance `t` along the ray.
    pub fn point_at(&self, t: f64) -> Vector3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ray = Ray::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(1.0, 1.0, 1.0));
        assert_eq!(ray.origin, Vector3::new(1.0, 2.0, 3.0));
        assert_eq!(
            ray.direction,
            Vector3::new(
                1.0 / 3.0_f64.sqrt(),
                1.0 / 3.0_f64.sqrt(),
                1.0 / 3.0_f64.sqrt()
            )
        );
    }

    #[test]
    fn test_point_at() {
        let ray = Ray::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(1.0, 1.0, 1.0));
        assert_eq!(
            ray.point_at(1.0),
            Vector3::new(
                1.0 + 1.0 / 3.0_f64.sqrt(),
                2.0 + 1.0 / 3.0_f64.sqrt(),
                3.0 + 1.0 / 3.0_f64.sqrt()
            )
        );

        assert_eq!(
            ray.point_at(2.0),
            Vector3::new(
                1.0 + 2.0 / 3.0_f64.sqrt(),
                2.0 + 2.0 / 3.0_f64.sqrt(),
                3.0 + 2.0 / 3.0_f64.sqrt()
            )
        );
    }
}
