use crate::math::ray::Ray;
use crate::math::point::Point;
use crate::math::plane::Plane;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn does_intersect(&self, line: Ray) -> bool {
        //plane is orthogonal to ray
        let p = Plane::from_normal_and_point(line.direction, &self.center);

        //get the point of intersect
        let point = line.intersection_point_with_plane(&p);

        //check if parralel and p is in circle
        match point{ 
            Some(p) => p.distance(&self.center) <= self.radius,
            None => false,
        }
    }

    ///returns None if the Sphere does not intersect with the point else the point is returned.
    ///the Point may be in opposite direction to the direction vector
    pub fn get_first_intersect_point(&self, line: &Ray) -> Option<Point> {
        let u = line.direction.get_unitvector();
        let o = line.location.local_vector();

        let c = self.center.local_vector();
        let r = self.radius;

        //some crazy math...
        let delta = u.dot_product(&(o-c)).powi(2) - ((o-c).dot_product(&(o-c))-r.powi(2));
        if delta < 0.0 {
            return None;
        }
        let d1 = -(u.dot_product(&(o-c))) + delta.sqrt();
        let d2 = -(u.dot_product(&(o-c))) - delta.sqrt();

        //get the closest intersect
        let d = if d1.abs() < d2.abs() {
            d1
        }else {
            d2
        };

        //construct intersection point
        let p = line.get_point_for_s_unitv(d);

        Some(p)
    }
}

#[cfg(test)]
mod test{
    use super::*; 
    use crate::math::vec::Vector3;

    #[test]
    fn test_get_first_intersect1(){
        let center = Point::new(2.0, 0.0, 0.0);
        let sphere = Sphere::new(center, 1.0);

        let location = Point::new(0.0, 0.0, 0.0);
        let direction = Vector3::new(3.0, 0.0, 0.0);

        let line = Ray::new(location, direction);

        let intersect = sphere.get_first_intersect_point(&line).unwrap();
        assert_eq!(intersect, Point::new(1.0, 0.0, 0.0))
    }

    #[test]
    fn test_get_first_intersect2(){
        let center = Point::new(-2.0, 0.0, 0.0);
        let sphere = Sphere::new(center, 1.0);

        let location = Point::new(0.0, 0.0, 0.0);
        let direction = Vector3::new(3.0, 0.0, 0.0);

        let line = Ray::new(location, direction);

        let intersect = sphere.get_first_intersect_point(&line).unwrap();
        assert_eq!(intersect, Point::new(-1.0, 0.0, 0.0))
    }

    #[test]
    fn test_get_first_intersect3(){
        let center = Point::new(2.0, 0.0, 0.0);
        let sphere = Sphere::new(center, 1.0);

        let location = Point::new(0.0, 1.0, 0.0);
        let direction = Vector3::new(2.0, 0.0, 0.0);

        let line = Ray::new(location, direction);

        let intersect = sphere.get_first_intersect_point(&line).unwrap();
        assert_eq!(intersect, Point::new(2.0, 1.0, 0.0))
    }

    #[test]
    fn test_get_first_intersect4(){
        let center = Point::new(2.0, 0.0, 0.0);
        let sphere = Sphere::new(center, 1.0);

        let location = Point::new(0.0, 0.0, 0.0);
        let direction = Vector3::new(2.0, 1.0, 0.0);

        let line = Ray::new(location, direction);

        let intersect = sphere.get_first_intersect_point(&line).unwrap();
        assert_eq!(intersect, Point::new(1.2000002, 0.6000001, 0.0))
    }
}
