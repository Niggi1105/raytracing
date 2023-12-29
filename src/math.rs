use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Clone, Copy)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy)]
struct Plane {
    normal: Vector3,
    a: f32,
}

#[derive(Clone, Copy)]
struct Line {
    location: Point,
    direction: Vector3,
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
        Self {
            x: b.x - a.x,
            y: b.y - a.y,
            z: b.z - a.z,
        }
    }

    pub fn stretch(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        self.stretch(1.0 / self.length())
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
        let mut n = plane.normal;
        n.normalize();
        let op = plane.get_point_on_plane().local_vector();
        let or = self.local_vector();

        n.dot_product(&(or - op)).abs()
    }
}

impl Line {
    pub fn new(location: Point, direction: Vector3) -> Self {
        Self {
            location,
            direction,
        }
    }

    pub fn form_points(p: Point, s: Point) -> Self{
        let dir = Vector3::ab_from_points(&p, &s);
        Self { location: p, direction: dir }
    }

}

impl Plane {
    pub fn new(normal: Vector3, a: f32) -> Self {
        Self { normal, a }
    }

    pub fn from_normal_and_point(normal: Vector3, point: &Point) -> Self {
        Self {
            normal,
            a: normal.x * point.x + normal.y * point.y + normal.z * point.z,
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
