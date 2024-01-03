use std::ops::{Add, AddAssign, Sub, SubAssign, Mul};

use crate::math::vec::Vector3;

use super::point::Point;

#[derive(Clone, Copy, Debug)]
pub struct Quat{
    a: f32,
    cplx: Vector3
}

impl Add for Quat{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            a: self.a + rhs.a,
            cplx: self.cplx + rhs.cplx,
        }
    }
}

impl AddAssign for Quat{
    fn add_assign(&mut self, rhs: Self) {
        self.a += rhs.a;
        self.cplx += rhs.cplx;
    }
}

impl Sub for Quat{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            a: self.a - rhs.a,
            cplx: self.cplx - rhs.cplx,
        }
    }
}

impl SubAssign for Quat{
    fn sub_assign(&mut self, rhs: Self) {
        self.a -= rhs.a;
        self.cplx -= rhs.cplx;
    }
}

impl Mul for Quat{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let a1 = self.a;
        let b1 = self.cplx.x;
        let c1 = self.cplx.y;
        let d1 = self.cplx.z;
        let a2 = rhs.a;
        let b2 = rhs.cplx.x;
        let c2 = rhs.cplx.y;
        let d2 = rhs.cplx.z;
        Self{
            a: a1 * a2 - b1 * b2 - c1 * c2 - d1 * d2,
            cplx: Vector3::new(
                a1 * b2 + b1 * a2 + c1 * d2 - d1 * c2,
                a1 * c2 - b1 * d2 + c1 * a2 + d1 * b2,
                a1 * d2 + b1 * c2 - c1 * b2 + d1 * a2,
            ) 
        }
    }
}

impl Quat{
    pub fn from_normal_and_angle(normal: Vector3, angle: f32) -> Self{
        let half_angle = (angle/2.0).to_radians();
        Self { a: half_angle.cos(), cplx: normal.stretched(half_angle.sin()) }
    }

    pub fn new(a: f32, cplx: Vector3) -> Self {
        Self { a, cplx }
    }

    pub fn inverse(&self) -> Self {
        Self { a: 1.0/self.a, cplx: self.cplx.inverse() }
    }

    pub fn conjugate(&self) -> Self{
        Self { a: self.a, cplx: self.cplx.negate() }
    }

    pub fn make_unitquarternion(&mut self) {
        let len = (self.a * self.a + self.cplx.x * self.cplx.x + self.cplx.y * self.cplx.y + self.cplx.z * self.cplx.z).sqrt();
        self.a /= len;
        self.cplx.stretch(1.0/len);
    }

    fn get_unitquarternion(&self) -> Self {
        let len = (self.a * self.a + self.cplx.x * self.cplx.x + self.cplx.y * self.cplx.y + self.cplx.z * self.cplx.z).sqrt();
        Self{
            a: self.a / len,
            cplx: self.cplx.stretched(1.0/len),
        }
    }

    fn to_point(self) -> Point{
        Point::new(self.cplx.x, self.cplx.y, self.cplx.z)
    }

    ///applies the rotation specified by the quarternion to the point
    pub fn rotate(&self, p: Point) -> Point{
        let q_point = Self::new(0.0, p.local_vector());
        let s = self.get_unitquarternion();
        let temp = s * q_point;
        let r = temp * s.conjugate();
        r.to_point()
    }
}


#[cfg(test)]
mod test{
    use crate::math::{vec::Vector3, point::Point};

    use super::Quat;

    #[test]
    fn test_rotation_around_y(){
        let q = Quat::from_normal_and_angle(Vector3::new(0.0, 1.0, 0.0), 90.0);
        let p = Point::new(1.0, 0.0, 0.0);
        let rotated = q.rotate(p);

        assert_eq!(rotated.x.round(), 0.0);
        assert_eq!(rotated.y.round(), 0.0);
        assert_eq!(rotated.z.round(), -1.0);
    }

    #[test]
    fn test_rotation_around_x(){
        let q = Quat::from_normal_and_angle(Vector3::new(1.0, 0.0, 0.0), 90.0);
        let p = Point::new(0.0, 1.0, 0.0);
        let rotated = q.rotate(p);

        assert_eq!(rotated.x.round(), 0.0);
        assert_eq!(rotated.y.round(), 0.0);
        assert_eq!(rotated.z.round(), 1.0);
    }

    #[test]
    fn test_rotation_around_x_5(){
        let q = Quat::from_normal_and_angle(Vector3::new(1.0, 0.0, 0.0), 90.0);
        let p = Point::new(0.0, 5.0, 0.0);
        let rotated = q.rotate(p);

        assert_eq!(rotated.x.round(), 0.0);
        assert_eq!(rotated.y.round(), 0.0);
        assert_eq!(rotated.z.round(), 5.0);
    }
    
}



