use std::ops::{Add, AddAssign, Sub, SubAssign};
use crate::math::point::Point;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn ab_from_points(a: &Point, b: &Point) -> Self {
        b.local_vector() - a.local_vector()
    }

    pub fn stretch(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }

    pub fn stretched(&self, factor: f32) -> Self {
        Self { 
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn make_unitvector(&mut self) {
        self.stretch(1.0 / self.length())
    }

    pub fn get_unitvector(&self) -> Self {
        self.stretched(1.0/self.length())
    }

    pub fn dot_product(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_product(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn angle(&self, other: &Self) -> f32 {
        (self.dot_product(other) / (self.length() * other.length())).cosh()
    }

    pub fn inverse(&self) -> Self {
        Self { x: 1.0/self.x, y: 1.0/self.y, z: 1.0/self.z }
    }

    pub fn negate(&self) -> Self{
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}
