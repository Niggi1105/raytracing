pub mod vec;
pub mod plane;
pub mod ray;
pub mod point;
pub mod quat;

#[cfg(test)]
mod test {
    use super::vec::Vector3;
    use super::plane::Plane;
    use super::ray::Ray;
    use super::point::Point;

    #[test]
    fn test_intersection_point() {
        let location = Point::new(1.0, 1.0, 2.0);
        let l = Ray::new(location, Vector3::new(1.0, 2.0, -1.0));
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
        let l = Ray::new(location, Vector3::new(1.0, 2.0, -1.0));
        let p = Plane::from_points(
            &Point::new(1.5, -2.2, 1.3),
            &Point::new(0.5, -2.0, 1.4),
            &Point::new(-1.0, -6.0, 0.0),
        );

        let result = Point::new( 0.72390115, 0.44780225, 2.276099);
        assert_eq!(l.intersection_point_with_plane(&p).unwrap(), result);
    }

    #[test]
    fn test_intersection_point_parralel() {
        let direction = Vector3::new(1.0, 1.0, 2.0);
        let l = Ray::new(Point::new(1.0, 2.0, -1.0), direction);

        let crossp = direction.cross_product(&Vector3::new(0.0, 0.0, 1.0));
        let p = Plane::from_normal_and_point(crossp, &Point::new(5.0, 5.0, 5.0));

        assert!(l.intersection_point_with_plane(&p).is_none());
    }
}
