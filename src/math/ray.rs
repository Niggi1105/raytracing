use crate::math::plane::Plane;
use crate::math::vec::Vector3;
use crate::math::point::Point;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub location: Point,
    pub direction: Vector3,
}

impl Ray {
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
