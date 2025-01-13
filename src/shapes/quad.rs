use crate::vector3::Vector3;

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

// impl Hittable for Quad {
//     fn hit(&self, ray: &Ray) -> Option<HitRecord> {}
// }
