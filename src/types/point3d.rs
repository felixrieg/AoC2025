#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point3D {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point3D {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Point3D { x, y, z }
    }

    pub fn distance_to(&self, other: &Point3D) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return None;
        }
        let x = parts[0].trim().parse().ok()?;
        let y = parts[1].trim().parse().ok()?;
        let z = parts[2].trim().parse().ok()?;
        Some(Point3D::new(x, y, z))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point2D {
    pub x: isize,
    pub y: isize,
}

impl Point2D {
    pub fn new(x: isize, y: isize) -> Self {
        Point2D { x, y }
    }

    #[allow(unused)]
    pub fn distance_to(&self, other: &Point2D) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return None;
        }
        let x = parts[0].trim().parse().ok()?;
        let y = parts[1].trim().parse().ok()?;
        Some(Point2D::new(x, y))
    }

    pub fn square_between(one: &Point2D, two: &Point2D) -> isize {
        let dx = (one.x - two.x).abs() + 1;
        let dy = (one.y - two.y).abs() + 1;
        dx * dy
    }

    pub fn points_between(one: &Point2D, two: &Point2D) -> Vec<Point2D> {
        let mut points = Vec::new();
        for x in one.x.min(two.x)..=one.x.max(two.x) {
            for y in one.y.min(two.y)..=one.y.max(two.y) {
                points.push(Point2D::new(x, y));
            }
        }
        points
    }

    pub fn inside_rectangle(&self, point1: &Point2D, point2: &Point2D) -> bool {
        let min_x = point1.x.min(point2.x);
        let max_x = point1.x.max(point2.x);
        let min_y = point1.y.min(point2.y);
        let max_y = point1.y.max(point2.y);
        self.x > min_x && self.x < max_x && self.y > min_y && self.y < max_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = Point3D::new(1, 2, 3);
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
        assert_eq!(p.z, 3);
    }

    #[test]
    fn test_distance_to_same_point() {
        let p1 = Point3D::new(0, 0, 0);
        let p2 = Point3D::new(0, 0, 0);
        assert_eq!(p1.distance_to(&p2), 0.0);
    }

    #[test]
    fn test_distance_to_axis_aligned() {
        let p1 = Point3D::new(0, 0, 0);
        let p2 = Point3D::new(3, 0, 0);
        assert_eq!(p1.distance_to(&p2), 3.0);

        let p3 = Point3D::new(0, 4, 0);
        assert_eq!(p1.distance_to(&p3), 4.0);

        let p4 = Point3D::new(0, 0, 5);
        assert_eq!(p1.distance_to(&p4), 5.0);
    }

    #[test]
    fn test_distance_to_3_4_5_triangle() {
        let p1 = Point3D::new(0, 0, 0);
        let p2 = Point3D::new(3, 4, 0);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_distance_to_3d_pythagorean() {
        let p1 = Point3D::new(0, 0, 0);
        let p2 = Point3D::new(2, 3, 6);
        assert_eq!(p1.distance_to(&p2), 7.0);
    }

    #[test]
    fn test_distance_to_negative_coordinates() {
        let p1 = Point3D::new(-1, -1, -1);
        let p2 = Point3D::new(1, 1, 1);
        let dist = p1.distance_to(&p2);
        assert!((dist - 3.464_101_615_137_754_6).abs() < 0.001);
    }

    #[test]
    fn test_distance_to_symmetry() {
        let p1 = Point3D::new(1, 2, 3);
        let p2 = Point3D::new(4, 5, 6);
        assert_eq!(p1.distance_to(&p2), p2.distance_to(&p1));
    }

    #[test]
    fn test_from_string_valid() {
        let s = "1,2,3";
        let p = Point3D::from_string(s);
        assert_eq!(p, Some(Point3D::new(1, 2, 3)));
    }

    #[test]
    fn test_from_string_with_spaces() {
        let s = "  1 , 2 , 3  ";
        let p = Point3D::from_string(s);
        assert_eq!(p, Some(Point3D::new(1, 2, 3)));
    }

    #[test]
    fn test_from_string_negative() {
        let s = "-5,-10,15";
        let p = Point3D::from_string(s);
        assert_eq!(p, Some(Point3D::new(-5, -10, 15)));
    }

    #[test]
    fn test_from_string_zero() {
        let s = "0,0,0";
        let p = Point3D::from_string(s);
        assert_eq!(p, Some(Point3D::new(0, 0, 0)));
    }

    #[test]
    fn test_from_string_invalid_too_few_parts() {
        let s = "1,2";
        let p = Point3D::from_string(s);
        assert_eq!(p, None);
    }

    #[test]
    fn test_from_string_invalid_too_many_parts() {
        let s = "1,2,3,4";
        let p = Point3D::from_string(s);
        assert_eq!(p, None);
    }

    #[test]
    fn test_from_string_invalid_non_numeric() {
        let s = "a,b,c";
        let p = Point3D::from_string(s);
        assert_eq!(p, None);
    }

    #[test]
    fn test_from_string_mixed_invalid() {
        let s = "1,2,abc";
        let p = Point3D::from_string(s);
        assert_eq!(p, None);
    }

    #[test]
    fn test_from_string_empty() {
        let s = "";
        let p = Point3D::from_string(s);
        assert_eq!(p, None);
    }

    #[test]
    fn test_equality() {
        let p1 = Point3D::new(1, 2, 3);
        let p2 = Point3D::new(1, 2, 3);
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_inequality() {
        let p1 = Point3D::new(1, 2, 3);
        let p2 = Point3D::new(1, 2, 4);
        assert_ne!(p1, p2);
    }

    #[test]
    fn test_clone() {
        let p1 = Point3D::new(1, 2, 3);
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_copy_semantics() {
        let p1 = Point3D::new(1, 2, 3);
        let p2 = p1;
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_large_coordinates() {
        let p1 = Point3D::new(isize::MAX / 2, isize::MAX / 2, isize::MAX / 2);
        let p2 = Point3D::new(-isize::MAX / 2, -isize::MAX / 2, -isize::MAX / 2);
        let _ = p1.distance_to(&p2);
    }

    #[test]
    fn test_from_string_large_numbers() {
        let s = "1000000,-500000,250000";
        let p = Point3D::from_string(s);
        assert_eq!(p, Some(Point3D::new(1_000_000, -500_000, 250_000)));
    }
}
