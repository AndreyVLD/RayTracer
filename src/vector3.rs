use image::Rgb;
use std::ops;

/// Represents a 3D vector.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector3 {
    /// The x-coordinate of the vector.
    pub x: f64,
    /// The y-coordinate of the vector.
    pub y: f64,
    /// The z-coordinate of the vector.
    pub z: f64,
}

impl Vector3 {
    /// Creates a new `Vector3` instance.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the vector.
    /// * `y` - The y-coordinate of the vector.
    /// * `z` - The z-coordinate of the vector.
    ///
    /// # Returns
    ///
    /// A new `Vector3` instance.
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Computes the length (magnitude) of the vector.
    ///
    /// # Returns
    ///
    /// The length of the vector.
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalizes the vector to have a length of 1.
    ///
    /// # Returns
    ///
    /// The normalized vector.
    pub fn normalize(&self) -> Vector3 {
        let len = self.length();
        if len == 0.0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        Vector3::new(self.x / len, self.y / len, self.z / len)
    }

    /// Computes the dot product of this vector and another vector.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The other vector.
    ///
    /// # Returns
    ///
    /// The dot product of the two vectors.
    pub fn dot(&self, rhs: &Vector3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Computes the cross product of this vector and another vector.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The other vector.
    ///
    /// # Returns
    ///
    /// The cross product of the two vectors.
    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        let x = (self.y * rhs.z) - (self.z * rhs.y);
        let y = (self.z * rhs.x) - (self.x * rhs.z);
        let z = (self.x * rhs.y) - (self.y * rhs.x);
        Vector3::new(x, y, z)
    }

    /// Converts the vector to an RGB color.
    ///
    /// # Returns
    ///
    /// The RGB color representation of the vector.
    pub fn to_rgb(self) -> Rgb<u8> {
        Rgb::from([self.x as u8, self.y as u8, self.z as u8])
    }

    /// Generates a random vector with each component in the given range.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value for each component.
    /// * `max` - The maximum value for each component.
    ///
    /// # Returns
    ///
    /// A random vector.
    pub fn random(min: f64, max: f64) -> Vector3 {
        Vector3::new(
            fastrand::f64() * (max - min) + min,
            fastrand::f64() * (max - min) + min,
            fastrand::f64() * (max - min) + min,
        )
    }

    /// Generates a random vector within a unit disk.
    ///
    /// # Returns
    ///
    /// A random vector within a unit disk.
    pub fn random_in_unit_disk() -> Vector3 {
        let theta = fastrand::f64() * std::f64::consts::PI * 2.0;
        let x = theta.cos();
        let y = theta.sin();
        Vector3::new(x, y, 0.0)
    }

    /// Generates a random vector within a unit sphere.
    ///
    /// # Returns
    ///
    /// A random vector within a unit sphere.
    pub fn random_in_unit_sphere() -> Vector3 {
        let azimuth = fastrand::f64() * 2.0 * std::f64::consts::PI;
        let polar = fastrand::f64() * std::f64::consts::PI;

        let x = polar.sin() * azimuth.cos();
        let y = polar.sin() * azimuth.sin();
        let z = polar.cos();
        Vector3::new(x, y, z)
    }

    /// Generates a random vector on the hemisphere defined by the given normal.
    ///
    /// # Arguments
    ///
    /// * `normal` - The normal vector defining the hemisphere.
    ///
    /// # Returns
    ///
    /// A random vector on the hemisphere.
    pub fn random_on_hemisphere(normal: &Vector3) -> Vector3 {
        let v = Vector3::random_in_unit_sphere();
        if v.dot(normal) > 0.0 {
            v
        } else {
            -v
        }
    }

    /// Checks if the vector is near zero in all components.
    ///
    /// # Returns
    ///
    /// `true` if the vector is near zero, `false` otherwise.
    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;

        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}

impl Default for Vector3 {
    /// Creates a default `Vector3` instance with all components set to zero.
    ///
    /// # Returns
    ///
    /// A default `Vector3` instance.
    fn default() -> Self {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

impl ops::Add for Vector3 {
    type Output = Vector3;

    /// Adds two vectors component-wise.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right-hand side vector.
    ///
    /// # Returns
    ///
    /// The resulting vector after addition.
    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::AddAssign for Vector3 {
    /// Adds another vector to this vector component-wise.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right-hand side vector.
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Mul for Vector3 {
    type Output = Vector3;

    /// Multiplies two vectors component-wise.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right-hand side vector.
    ///
    /// # Returns
    ///
    /// The resulting vector after multiplication.
    fn mul(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;

    /// Subtracts another vector from this vector component-wise.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right-hand side vector.
    ///
    /// # Returns
    ///
    /// The resulting vector after subtraction.
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    /// Negates the vector.
    ///
    /// # Returns
    ///
    /// The negated vector.
    fn neg(self) -> Self::Output {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    /// Multiplies the vector by a scalar.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The scalar value.
    ///
    /// # Returns
    ///
    /// The resulting vector after multiplication.
    fn mul(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<u32> for Vector3 {
    type Output = Vector3;

    /// Multiplies the vector by an integer scalar.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The integer scalar value.
    ///
    /// # Returns
    ///
    /// The resulting vector after multiplication.
    fn mul(self, rhs: u32) -> Self::Output {
        Vector3::new(
            self.x * rhs as f64,
            self.y * rhs as f64,
            self.z * rhs as f64,
        )
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    /// Divides the vector by a scalar.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The scalar value.
    ///
    /// # Returns
    ///
    /// The resulting vector after division.
    fn div(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    /// Multiplies a scalar by a vector.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The vector.
    ///
    /// # Returns
    ///
    /// The resulting vector after multiplication.
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}
impl ops::Mul<Vector3> for u32 {
    type Output = Vector3;

    /// Multiplies an integer scalar by a vector.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The vector.
    ///
    /// # Returns
    ///
    /// The resulting vector after multiplication.
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(
            self as f64 * rhs.x,
            self as f64 * rhs.y,
            self as f64 * rhs.z,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::vector3::Vector3;

    #[test]
    fn test_new() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_length() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.length().powf(2.0), 14.0);
    }

    #[test]
    fn test_normalize() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(
            v.normalize(),
            Vector3::new(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            )
        );
    }

    #[test]
    fn test_dot() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.dot(&Vector3::new(2.0, 5.0, 6.0)), 30.0);
    }

    #[test]
    fn test_add() {
        let left = Vector3::new(1.0, 2.0, 3.0);
        let right = Vector3::new(2.0, 4.0, 6.0);
        let solution = Vector3::new(3.0, 6.0, 9.0);
        assert_eq!(left + right, solution);
        assert_eq!(right + left, solution);
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vector3::new(1.0, 2.0, 3.0);
        v += Vector3::new(2.0, 5.0, 13.0);
        assert_eq!(v, Vector3::new(3.0, 7.0, 16.0));
    }

    #[test]
    fn test_sub() {
        let left = Vector3::new(1.0, 2.0, 3.0);
        let right = Vector3::new(2.0, 4.0, 6.0);
        let solution1 = Vector3::new(-1.0, -2.0, -3.0);
        let solution2 = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(left - right, solution1);
        assert_eq!(right - left, solution2);
    }

    #[test]
    fn test_neg() {
        let left = Vector3::new(1.0, 2.0, 3.0);
        let right = Vector3::new(-1.0, -2.0, -3.0);
        assert_eq!(-left, right);
    }

    #[test]
    fn test_mul_scalar() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let solution = Vector3::new(2.0, 4.0, 6.0);

        assert_eq!(v * 2.0, solution);
        assert_eq!(2.0 * v, solution);
    }

    #[test]
    fn test_mul_scalar_int() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let solution = Vector3::new(2.0, 4.0, 6.0);

        assert_eq!(v * 2, solution);
        assert_eq!(2 * v, solution);
    }

    #[test]
    fn test_mul_vector3() {
        let left = Vector3::new(1.0, 2.0, 3.0);
        let right = Vector3::new(0.5, 1.0, 2.0);
        let solution = Vector3::new(0.5, 2.0, 6.0);
        assert_eq!(left * right, solution);
        assert_eq!(right * left, solution);
    }

    #[test]
    fn test_div_scalar() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let solution = Vector3::new(0.5, 1.0, 1.5);
        assert_eq!(v / 2.0, solution);
    }

    #[test]
    fn test_random() {
        let v = Vector3::random(0.0, 1.0);
        assert!(v.x <= 1.0 && v.x >= 0.0);
        assert!(v.y <= 1.0 && v.y >= 0.0);
        assert!(v.z <= 1.0 && v.z >= 0.0);
    }

    #[test]
    fn test_random_in_unit_sphere() {
        let v = Vector3::random_in_unit_sphere();
        assert!(0.999 <= v.length() && v.length() <= 1.0);
    }
}
