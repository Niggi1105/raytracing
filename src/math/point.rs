use crate::math::vec::Vector3;
use crate::math::plane::Plane;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn local_vector(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn distance(&self, other: &Self) -> f32 {
        Vector3::ab_from_points(self, other).length()
    }

    pub fn distance_to_plane(&self, plane: Plane) -> f32 {
        //Hessesche Normalform
        ((self.local_vector().dot_product(&plane.normal) - plane.a) / plane.normal.length()).abs()
    }
}

