use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    normal: Vector3,
    a: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    pub location: Point,
    pub direction: Vector3,
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

impl Line {
    pub fn new(location: Point, direction: Vector3) -> Self {
        Self {
            location,
            direction,
        }
    }

    pub fn form_points(p: Point, s: Point) -> Self {
        let dir = Vector3::ab_from_points(&p, &s);
        Self {
            location: p,
            direction: dir,
        }
    }

    ///returns None if the plane and the line are parralel (doesnt check for identical)
    pub fn intersection_point_with_plane(&self, plane: &Plane) -> Option<Point> {
        if plane.normal.dot_product(&self.direction) == 0.0 {
            return None;
        };
        let n = plane.normal;
        let d = self.direction;
        let l = self.location.local_vector();

        //some complicated math...
        let s = (plane.a - n.dot_product(&l)) / n.dot_product(&d);

        Some(Point::new(l.x + s * d.x, l.y + s * d.y, l.z + s * d.z))
    }

    pub fn minimum_distance_to_other(&self, other: Self) -> f32 {
        //can construct two parralel planes form lines and calculate their distance
        let n = self.direction.cross_product(&other.direction);
        let p = Plane::from_normal_and_point(n, &self.location);
        other.location.distance_to_plane(p)
    }

    ///uses the direction vector as is to find the point
    pub fn get_point_for_s(&self, s: f32) -> Point {
        Point::new(
            self.location.x + s * self.direction.x, 
            self.location.y + s * self.direction.y, 
            self.location.z + s * self.direction.z, 
        )
    }

    ///uses the unit vector of the direction to find the point
    pub fn get_point_for_s_unitv(&self, s: f32) -> Point {
        let dir = self.direction.get_unitvector();
        Point::new(
            self.location.x + s * dir.x, 
            self.location.y + s * dir.y, 
            self.location.z + s * dir.z, 
        )
    }

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersection_point() {
        let location = Point::new(1.0, 1.0, 2.0);
        let l = Line::new(location, Vector3::new(1.0, 2.0, -1.0));
        let p = Plane::from_points(
            &location,
            &Point::new(0.5, -2.0, 1.4),
            &Point::new(-1.0, -6.0, 0.0),
        );

        assert_eq!(l.intersection_point_with_plane(&p).unwrap(), location);
    }

    #[test]
    fn test_intersection_point_ouch() {
        let location = Point::new(1.0, 1.0, 2.0);
        let l = Line::new(location, Vector3::new(1.0, 2.0, -1.0));
        let p = Plane::from_points(
            &Point::new(1.5, -2.2, 1.3),
            &Point::new(0.5, -2.0, 1.4),
            &Point::new(-1.0, -6.0, 0.0),
        );

        let result = Point {
            x: 0.72390115,
            y: 0.44780225,
            z: 2.276099,
        };
        assert_eq!(l.intersection_point_with_plane(&p).unwrap(), result);
    }

    #[test]
    fn test_intersection_point_parralel() {
        let direction = Vector3::new(1.0, 1.0, 2.0);
        let l = Line::new(Point::new(1.0, 2.0, -1.0), direction);

        let crossp = direction.cross_product(&Vector3::new(0.0, 0.0, 1.0));
        let p = Plane::from_normal_and_point(crossp, &Point::new(5.0, 5.0, 5.0));

        assert!(l.intersection_point_with_plane(&p).is_none());
    }
}
