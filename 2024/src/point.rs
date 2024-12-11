use std::ops::{Add, Neg, Sub};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[must_use]
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        (0..=x).contains(&self.x) && (0..=y).contains(&self.y)
    }

    pub fn generate_sequence(
        self,
        delta: Self,
        x: i32,
        y: i32,
    ) -> impl Iterator<Item = Self> + 'static {
        std::iter::successors(Some(self), move |&point| {
            let next = point + delta;
            next.in_bounds(x, y).then_some(next)
        })
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        let x = i32::try_from(x).expect("x is too large");
        let y = i32::try_from(y).expect("y is too large");
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let p1 = Point::from((1, 2));
        assert_eq!(p1.x, 1);
        assert_eq!(p1.y, 2);

        let p2: Point = (3_usize, 4_usize).into();
        assert_eq!(p2.x, 3);
        assert_eq!(p2.y, 4);
    }

    #[test]
    #[should_panic(expected = "x is too large")]
    fn test_point_from_large_usize() {
        let x = usize::MAX;
        let _p = Point::from((x, 0_usize));
    }

    #[test]
    fn test_point_addition() {
        let p1 = Point::from((1, 2));
        let p2 = Point::from((3, 4));
        let sum = p1 + p2;
        assert_eq!(sum, Point::from((4, 6)));
    }

    #[test]
    fn test_point_subtraction() {
        let p1 = Point::from((5, 7));
        let p2 = Point::from((2, 3));
        let diff = p1 - p2;
        assert_eq!(diff, Point::from((3, 4)));
    }

    #[test]
    fn test_point_negation() {
        let p = Point::from((1, -2));
        let neg = -p;
        assert_eq!(neg, Point::from((-1, 2)));
    }

    #[test]
    fn test_point_in_bounds() {
        let p = Point::from((5, 5));
        assert!(p.in_bounds(10, 10));
        assert!(p.in_bounds(5, 5));
        assert!(!p.in_bounds(4, 10));
        assert!(!p.in_bounds(10, 4));
    }

    #[test]
    fn test_point_sequence_generation() {
        let start = Point::from((0, 0));
        let delta = Point::from((1, 1));
        let sequence: Vec<Point> = start.generate_sequence(delta, 2, 2).collect();

        assert_eq!(
            sequence,
            vec![
                Point::from((0, 0)),
                Point::from((1, 1)),
                Point::from((2, 2))
            ]
        );
    }

    #[test]
    fn test_point_equality() {
        let p1 = Point::from((1, 2));
        let p2 = Point::from((1, 2));
        let p3 = Point::from((2, 1));

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_point_copy() {
        let p1 = Point::from((1, 2));
        let p2 = p1;
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_zero_bounds() {
        let p = Point::from((0, 0));
        assert!(p.in_bounds(0, 0));
        assert!(p.in_bounds(1, 1));
        assert!(!p.in_bounds(-1, -1));
    }
}
