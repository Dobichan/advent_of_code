use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use crate::direction::Direction;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub const ORIGIN: Self = Self { x: 0, y: 0 };

    pub const CARDINAL: [Point; 4] = [
        Direction::North.delta(),
        Direction::East.delta(),
        Direction::South.delta(),
        Direction::West.delta(),
    ];

    pub const DIAGONAL: [Point; 4] = [
        Point::new(1, -1),
        Point::new(1, 1),
        Point::new(-1, 1),
        Point::new(-1, -1),
    ];

    pub const NEIGHBORS: [Point; 8] = {
        let mut all = [Point::ORIGIN; 8];
        let mut i = 0;
        while i < 4 {
            all[i] = Self::CARDINAL[i];
            all[i + 4] = Self::DIAGONAL[i];
            i += 1
        }
        all
    };

    pub fn manhattan_distance(self, other: Self) -> isize {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    pub fn distance_squared(self, other: Self) -> isize {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        dx * dx + dy * dy
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        self + rhs.delta()
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        *self += rhs.delta();
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point3 {
    pub const fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub const ORIGIN: Self = Self { x: 0, y: 0, z: 0 };

    pub fn manhattan_distance(self, other: Self) -> isize {
        (other.x - self.x).abs() + (other.y - self.y).abs() + (other.z - self.z).abs()
    }

    pub fn distance_squared(self, other: Self) -> isize {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;

        dx * dx + dy * dy + dz * dz
    }
}

impl Add<Point3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Point3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl From<(isize, isize, isize)> for Point3 {
    fn from(value: (isize, isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_add_sub() {
        let a = Point::new(1, 2);
        let b = Point::new(10, 20);
        assert_eq!(a + b, Point::new(11, 22));
        assert_eq!(b - a, Point::new(9, 18));
        assert_eq!((a + b) - b, a);
    }

    #[test]
    fn test_assign_ops() {
        let mut p = Point::new(1, 1);
        p += Point::new(2, 3);
        assert_eq!(p, Point::new(3, 4));
        p -= Point::new(1, 1);
        assert_eq!(p, Point::new(2, 3));
        p += Direction::North;
        assert_eq!(p, Point::new(2, 2));
    }

    #[test]
    fn test_mul_and_neg() {
        let p = Point::new(2, -3);
        assert_eq!(p * 3, Point::new(6, -9));
        assert_eq!(-p, Point::new(-2, 3));
        assert_eq!(p * -1, -p);
    }

    #[test]
    fn test_add_direction() {
        let p = Point::new(5, 5);
        assert_eq!(p + Direction::North, Point::new(5, 4));
        assert_eq!(p + Direction::East, Point::new(6, 5));
        assert_eq!(p + Direction::South, Point::new(5, 6));
        assert_eq!(p + Direction::West, Point::new(4, 5));
    }

    #[test]
    fn test_from_tuples() {
        assert_eq!(Point::from((3isize, -4isize)), Point::new(3, -4));
        assert_eq!(Point::from((3usize, 4usize)), Point::new(3, 4));
        let p: Point = (1isize, 2isize).into();
        assert_eq!(p, Point::new(1, 2));
    }

    #[test]
    fn test_distances() {
        let a = Point::new(1, 1);
        let b = Point::new(4, 5);
        assert_eq!(a.manhattan_distance(b), 7);
        assert_eq!(b.manhattan_distance(a), 7);
        assert_eq!(a.distance_squared(b), 25);
        assert_eq!(a.manhattan_distance(a), 0);
    }

    #[test]
    fn test_neighbour_constants() {
        assert_eq!(Point::CARDINAL, Direction::ALL.map(Direction::delta));
        assert_eq!(&Point::NEIGHBORS[..4], &Point::CARDINAL);
        assert_eq!(&Point::NEIGHBORS[4..], &Point::DIAGONAL);

        let set: HashSet<_> = Point::NEIGHBORS.into_iter().collect();
        assert_eq!(set.len(), 8);
        assert!(!set.contains(&Point::ORIGIN));
        assert!(
            Point::NEIGHBORS
                .iter()
                .all(|p| p.x.abs() <= 1 && p.y.abs() <= 1)
        );
    }

    #[test]
    fn test_point3() {
        let a = Point3::new(1, 2, 3);
        let b = Point3::new(4, 6, 3);
        assert_eq!(a + b, Point3::new(5, 8, 6));
        assert_eq!(b - a, Point3::new(3, 4, 0));
        assert_eq!(a.manhattan_distance(b), 7);
        assert_eq!(a.distance_squared(b), 25);
        assert_eq!(a.distance_squared(a), 0);
        assert_eq!(Point3::from((1isize, 2isize, 3isize)), a);
        assert_eq!(Point3::default(), Point3::ORIGIN);
    }
}
