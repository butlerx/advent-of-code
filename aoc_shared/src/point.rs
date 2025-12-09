use std::{
    fmt::Display,
    ops::{Add, Neg, Sub},
};

static DIRECTIONS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// A 2D point with integer coordinates.
///
/// Represents a point in 2D space with `x` and `y` coordinates. Provides utility methods for
/// common operations such as distance calculation, neighbor finding, and sequence generation.
///
/// # Examples
///
/// ```
/// use aoc_shared::Point;
/// let p = Point::new(1, 2);
/// assert_eq!(p.x, 1);
/// assert_eq!(p.y, 2);
/// ```
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    /// Creates a new `Point` with the given `x` and `y` coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_shared::Point;
    /// let p = Point::new(3, 4);
    /// assert_eq!(p.x, 3);
    /// assert_eq!(p.y, 4);
    /// ```
    #[must_use]
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    /// Returns `true` if the point is within the bounds `(0..=x, 0..=y)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_shared::Point;
    /// let p = Point::new(2, 3);
    /// assert!(p.in_bounds(5, 5));
    /// assert!(!p.in_bounds(1, 1));
    /// ```
    #[must_use]
    pub fn in_bounds(&self, x: i64, y: i64) -> bool {
        (0..=x).contains(&self.x) && (0..=y).contains(&self.y)
    }

    /// Generates a sequence of points starting from this point, stepping by `delta`,
    /// and stopping when the next point would be out of bounds `(0..=x, 0..=y)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_shared::Point;
    /// let start = Point::new(0, 0);
    /// let delta = Point::new(1, 1);
    /// let points: Vec<_> = start.generate_sequence(delta, 2, 2).collect();
    /// assert_eq!(points, vec![Point::new(0, 0), Point::new(1, 1), Point::new(2, 2)]);
    /// ```
    pub fn generate_sequence(
        self,
        delta: Self,
        x: i64,
        y: i64,
    ) -> impl Iterator<Item = Self> + 'static {
        std::iter::successors(Some(self), move |&point| {
            let next = point + delta;
            next.in_bounds(x, y).then_some(next)
        })
    }

    /// Returns the Manhattan distance between this point and another.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_shared::Point;
    /// let a = Point::new(1, 2);
    /// let b = Point::new(4, 6);
    /// assert_eq!(a.manhattan_distance(b), 7);
    /// ```
    #[must_use]
    pub fn manhattan_distance(self, b: Point) -> i64 {
        (self.x - b.x).abs() + (self.y - b.y).abs()
    }

    /// Returns the area distance between this point and another.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_shared::Point;
    /// let a = Point::new(1, 2);
    /// let b = Point::new(4, 6);
    /// assert_eq!(a.area_distance(b), 20);
    /// ```
    #[must_use]
    pub fn area_distance(self, b: Point) -> i64 {
        let width = (b.x - self.x).abs();
        let height = (b.y - self.y).abs();

        if width > 0 && height > 0 {
            (width + 1) * (height + 1)
        } else {
            0
        }
    }

    /// Returns the four cardinal neighbors (up, down, left, right) of this point.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_shared::Point;
    /// let p = Point::new(1, 1);
    /// let n = p.neighbours();
    /// assert!(n.contains(&Point::new(0, 1)));
    /// assert!(n.contains(&Point::new(2, 1)));
    /// assert!(n.contains(&Point::new(1, 0)));
    /// assert!(n.contains(&Point::new(1, 2)));
    /// ```
    #[must_use]
    pub fn neighbours(&self) -> Vec<Self> {
        vec![
            Self {
                x: self.x - 1,
                y: self.y,
            },
            Self {
                x: self.x + 1,
                y: self.y,
            },
            Self {
                x: self.x,
                y: self.y - 1,
            },
            Self {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }

    /// Returns all eight neighbors (including diagonals) of this point.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_shared::Point;
    /// let p = Point::new(1, 1);
    /// let n = p.neighbours_all_directions();
    /// assert_eq!(n.len(), 8);
    /// ```
    #[inline]
    #[must_use]
    pub fn neighbours_all_directions(&self) -> Vec<Self> {
        DIRECTIONS
            .iter()
            .map(|(dx, dy)| Self {
                x: self.x + dx,
                y: self.y + dy,
            })
            .collect()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

/// Converts a tuple `(x, y)` into a `Point`.
///
/// # Panics
///
/// Panics if the values cannot be converted to `i64`.
///
/// # Examples
///
/// ```
/// use aoc_shared::Point;
/// let p: Point = (3, 4).into();
/// assert_eq!(p, Point::new(3, 4));
/// ```
impl<T> From<(T, T)> for Point
where
    T: TryInto<i64>,
    T::Error: std::fmt::Debug,
{
    fn from((x, y): (T, T)) -> Self {
        let x = x.try_into().expect("x is too large");
        let y = y.try_into().expect("y is too large");
        Self { x, y }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let parts = s.trim().split_once(',').expect("Invalid input line");
        let x = parts.0.parse::<i64>().expect("Invalid x coordinate");
        let y = parts.1.parse::<i64>().expect("Invalid y coordinate");
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
