use crate::math::vec::Vector3;
use crate::math::point::Point;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    pub normal: Vector3,
    pub a: f32,
}

impl Plane {
    pub fn new(normal: Vector3, a: f32) -> Self {
        Self { normal, a }
    }

    pub fn from_normal_and_point(normal: Vector3, point: &Point) -> Self {
        Self {
            normal,
            a: normal.dot_product(&point.local_vector()),
        }
    }

    pub fn from_points(p: &Point, q: &Point, s: &Point) -> Self {
        let v1 = Vector3::ab_from_points(p, q);
        let v2 = Vector3::ab_from_points(p, s);

        let normal = v1.cross_product(&v2);
        Self::from_normal_and_point(normal, p)
    }

    pub fn point_is_on_plane(&self, point: &Point) -> bool {
        self.normal.x * point.x + self.normal.y * point.y + self.normal.z * point.z == self.a
    }

    pub fn get_point_on_plane(&self) -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: self.a / self.normal.z,
        }
    }

    pub fn angle_to_vec(&self, vector: &Vector3) -> f32 {
        (self.normal.dot_product(vector) / (self.normal.length() * vector.length())).sinh()
    }
}

