use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

use crate::points::Number;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point3D<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Number> Point3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Point3D { x, y, z }
    }
}

impl<T: Number> Add for Point3D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Number> Sub for Point3D<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Number> fmt::Display for Point3D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T: Number> Point3D<T> {
    pub fn manhattan_distance(&self, other: &Self) -> T {
        let dx = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let dy = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        let dz = if self.z > other.z {
            self.z - other.z
        } else {
            other.z - self.z
        };
        dx + dy + dz
    }
}

impl Point3D<i32> {
    pub fn distance_squared(&self, other: &Self) -> i64 {
        let dx = i64::from(self.x - other.x);
        let dy = i64::from(self.y - other.y);
        let dz = i64::from(self.z - other.z);
        dx * dx + dy * dy + dz * dz
    }
}

impl Point3D<i64> {
    pub fn distance_squared(&self, other: &Self) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

impl Point3D<f64> {
    pub fn euclidean_distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl Point3D<f32> {
    pub fn euclidean_distance(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl<T: Number> From<(T, T, T)> for Point3D<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Point3D { x, y, z }
    }
}

impl<T: Number> From<&(T, T, T)> for Point3D<T> {
    fn from(&(x, y, z): &(T, T, T)) -> Self {
        Point3D { x, y, z }
    }
}

impl<T: Number> From<[T; 3]> for Point3D<T> {
    fn from([x, y, z]: [T; 3]) -> Self {
        Point3D { x, y, z }
    }
}

impl<T: Number> From<Point3D<T>> for (T, T, T) {
    fn from(point: Point3D<T>) -> (T, T, T) {
        (point.x, point.y, point.z)
    }
}

impl<T> FromStr for Point3D<T>
where
    T: Number + FromStr,
    T::Err: std::fmt::Debug,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() < 3 {
            return Err("Invalid point3d format".to_string());
        }

        let x = parts[0]
            .parse()
            .map_err(|e| format!("Failed to parse x coordinate: {e:?}"))?;
        let y = parts[1]
            .parse()
            .map_err(|e| format!("Failed to parse y coordinate: {e:?}"))?;
        let z = parts[2]
            .parse()
            .map_err(|e| format!("Failed to parse z coordinate: {e:?}"))?;

        Ok(Point3D { x, y, z })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point3d_new() {
        let p = Point3D::new(1, 2, 3);
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
        assert_eq!(p.z, 3);

        let p_float = Point3D::new(1.0f32, 2.0f32, 3.0f32);
        let p_int64 = Point3D::new(1i64, 2i64, 3i64);
        assert!((p_float.x - 1.0f32).abs() < f32::EPSILON);
        assert_eq!(p_int64.x, 1i64);
    }

    #[test]
    fn test_point3d_add() {
        let p1 = Point3D::new(1, 2, 3);
        let p2 = Point3D::new(4, 5, 6);
        let result = p1 + p2;
        assert_eq!(result, Point3D::new(5, 7, 9));

        let p3 = Point3D::new(-1, -2, -3);
        let p4 = Point3D::new(1, 2, 3);
        assert_eq!(p3 + p4, Point3D::new(0, 0, 0));

        let p5 = Point3D::new(1.5f64, 2.5f64, 3.5f64);
        let p6 = Point3D::new(1.5f64, 2.5f64, 3.5f64);
        assert_eq!(p5 + p6, Point3D::new(3.0f64, 5.0f64, 7.0f64));
    }

    #[test]
    fn test_point3d_sub() {
        let p1 = Point3D::new(4, 5, 6);
        let p2 = Point3D::new(1, 2, 3);
        let result = p1 - p2;
        assert_eq!(result, Point3D::new(3, 3, 3));

        let p3 = Point3D::new(-1, -2, -3);
        let p4 = Point3D::new(1, 2, 3);
        assert_eq!(p3 - p4, Point3D::new(-2, -4, -6));

        let p5 = Point3D::new(5.0f64, 6.0f64, 7.0f64);
        let p6 = Point3D::new(2.0f64, 3.0f64, 4.0f64);
        assert_eq!(p5 - p6, Point3D::new(3.0f64, 3.0f64, 3.0f64));
    }

    #[test]
    fn test_point3d_display() {
        let p = Point3D::new(1, 2, 3);
        assert_eq!(format!("{p}"), "(1, 2, 3)");

        let p_float = Point3D::new(1.5f64, 2.5f64, 3.5f64);
        assert_eq!(format!("{p_float}"), "(1.5, 2.5, 3.5)");
    }

    #[test]
    fn test_manhattan_distance() {
        let p1 = Point3D::new(1, 1, 1);
        let p2 = Point3D::new(4, 5, 7);
        assert_eq!(p1.manhattan_distance(&p2), 13);

        let p3 = Point3D::new(-1, -1, -1);
        let p4 = Point3D::new(2, 2, 2);
        assert_eq!(p3.manhattan_distance(&p4), 9);

        let p5 = Point3D::new(1.0f64, 1.0f64, 1.0f64);
        let p6 = Point3D::new(4.0f64, 5.0f64, 7.0f64);
        assert!((p5.manhattan_distance(&p6) - 13.0f64).abs() < f64::EPSILON);

        let p7 = Point3D::new(1, 1, 1);
        let p8 = Point3D::new(1, 1, 1);
        assert_eq!(p7.manhattan_distance(&p8), 0);
    }

    #[test]
    fn test_distance_squared() {
        let p1 = Point3D::new(1i32, 2i32, 3i32);
        let p2 = Point3D::new(4i32, 6i32, 8i32);
        assert_eq!(p1.distance_squared(&p2), 50);

        let p3 = Point3D::new(0i32, 0i32, 0i32);
        let p4 = Point3D::new(1i32, 1i32, 1i32);
        assert_eq!(p3.distance_squared(&p4), 3);

        let p5 = Point3D::new(-1i32, -1i32, -1i32);
        let p6 = Point3D::new(1i32, 1i32, 1i32);
        assert_eq!(p5.distance_squared(&p6), 12);

        let p7 = Point3D::new(1i64, 2i64, 3i64);
        let p8 = Point3D::new(4i64, 6i64, 8i64);
        assert_eq!(p7.distance_squared(&p8), 50);
    }

    #[test]
    fn test_euclidean_distance() {
        let p1 = Point3D::new(1.0f64, 2.0f64, 3.0f64);
        let p2 = Point3D::new(4.0f64, 6.0f64, 8.0f64);
        let expected = 50.0f64.sqrt();
        assert!((p1.euclidean_distance(&p2) - expected).abs() < f64::EPSILON);

        let p3 = Point3D::new(0.0f32, 0.0f32, 0.0f32);
        let p4 = Point3D::new(1.0f32, 1.0f32, 1.0f32);
        let expected_f32 = 3.0f32.sqrt();
        assert!((p3.euclidean_distance(&p4) - expected_f32).abs() < f32::EPSILON);
    }

    #[test]
    fn test_from_tuple() {
        let tuple = (1, 2, 3);
        let point: Point3D<i32> = Point3D::from(tuple);
        assert_eq!(point, Point3D::new(1, 2, 3));

        let tuple_ref = &(4, 5, 6);
        let point: Point3D<i32> = Point3D::from(tuple_ref);
        assert_eq!(point, Point3D::new(4, 5, 6));

        let float_tuple = (1.5f64, 2.5f64, 3.5f64);
        let point: Point3D<f64> = Point3D::from(float_tuple);
        assert_eq!(point, Point3D::new(1.5f64, 2.5f64, 3.5f64));
    }

    #[test]
    fn test_from_array() {
        let array = [1, 2, 3];
        let point: Point3D<i32> = Point3D::from(array);
        assert_eq!(point, Point3D::new(1, 2, 3));
    }

    #[test]
    fn test_into_tuple() {
        let point = Point3D::new(1, 2, 3);
        let tuple: (i32, i32, i32) = point.into();
        assert_eq!(tuple, (1, 2, 3));
    }

    #[test]
    fn test_point3d_equality() {
        let p1 = Point3D::new(1, 2, 3);
        let p2 = Point3D::new(1, 2, 3);
        let p3 = Point3D::new(3, 2, 1);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);

        let p4 = Point3D::new(1.0f64, 2.0f64, 3.0f64);
        let p5 = Point3D::new(1.0f64, 2.0f64, 3.0f64);
        assert_eq!(p4, p5);
    }

    #[test]
    fn test_point3d_copy_clone() {
        let p1 = Point3D::new(1, 2, 3);
        let p2 = p1;
        assert_eq!(p1, p2);

        let p3 = p1;
        assert_eq!(p1, p3);
    }

    #[test]
    fn test_point3d_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Point3D::new(1, 2, 3));
        set.insert(Point3D::new(1, 2, 3));
        set.insert(Point3D::new(3, 2, 1));

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_from_str() {
        let point: Point3D<i32> = "1 2 3".parse().unwrap();
        assert_eq!(point, Point3D::new(1, 2, 3));

        let point: Point3D<f64> = "1.5 2.5 3.5".parse().unwrap();
        assert_eq!(point, Point3D::new(1.5f64, 2.5f64, 3.5f64));

        let result: Result<Point3D<i32>, _> = "1 2".parse();
        assert!(result.is_err());
    }
}
