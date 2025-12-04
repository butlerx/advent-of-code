use crate::Point;
use std::{cmp::PartialEq, iter::Iterator};

/// A 2D grid of values.
///
/// Stores values in a 2D grid with fixed width and height. Provides methods for
/// accessing, setting, and iterating over grid elements using [`Point`] coordinates.
///
/// # Examples
///
/// ```
/// use aoc_shared::{Grid, Point};
/// let grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
/// assert_eq!(grid.get(Point::new(1, 0)), Some(2));
/// ```
#[derive(Debug, Clone)]
pub struct Grid<T> {
    cells: Vec<T>,
    pub height: usize,
    pub width: usize,
}

pub struct IterGridState<'a, T> {
    grid: &'a Grid<T>,
    current: usize,
}

impl<'a, T> Iterator for IterGridState<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.grid.cells.len() {
            None
        } else {
            let y = self.current / self.grid.width;
            let x = self.current % self.grid.width;
            let point = Point::from((x, y));
            let value = &self.grid.cells[self.current];
            self.current += 1;
            Some((point, value))
        }
    }
}

impl<T> Grid<T> {
    /// Returns an iterator over all points and their values in the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_shared::{Grid, Point};
    /// let grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
    /// for (point, value) in grid.iter() {
    ///     // Use point and value
    /// }
    /// ```
    #[must_use]
    pub fn iter(&self) -> IterGridState<'_, T> {
        IterGridState {
            grid: self,
            current: 0,
        }
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = (Point, &'a T);
    type IntoIter = IterGridState<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Clone + Copy> From<Vec<Vec<T>>> for Grid<T> {
    fn from(nested: Vec<Vec<T>>) -> Self {
        let height = nested.len();
        let width = nested.first().map_or(0, std::vec::Vec::len);

        let cells: Vec<T> = nested
            .into_iter()
            .flat_map(std::iter::IntoIterator::into_iter)
            .collect();

        Self {
            cells,
            height,
            width,
        }
    }
}

impl<T: Clone + Copy> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        let mut iter = iter.into_iter().peekable();
        let width = iter.peek().map_or(0, std::vec::Vec::len);
        let mut height = 0;
        let cells: Vec<T> = iter
            .inspect(|_| height += 1)
            .flat_map(std::iter::IntoIterator::into_iter)
            .collect();

        Self {
            cells,
            height,
            width,
        }
    }
}

impl<T: Clone + Copy + PartialEq> Grid<T> {
    /// Creates a new grid of size `(pos.x + 1, pos.y + 1)` filled with the given value.
    ///
    /// # Panics
    ///
    /// Panics if `pos.x` or `pos.y` is negative or too large to convert to `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_shared::{Grid, Point};
    /// let grid = Grid::new(Point::new(1, 1), 0);
    /// assert_eq!(grid.width, 2);
    /// assert_eq!(grid.height, 2);
    /// ```
    pub fn new(pos: Point, fill: T) -> Self {
        let width = usize::try_from(pos.x).expect("number too large") + 1;
        let height = usize::try_from(pos.y).expect("number too large") + 1;
        let cells = vec![fill; width * height];
        Self {
            cells,
            height,
            width,
        }
    }

    /// Returns the value at the given point, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_shared::{Grid, Point};
    /// let grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
    /// assert_eq!(grid.get(Point::new(1, 0)), Some(2));
    /// assert_eq!(grid.get(Point::new(2, 0)), None);
    /// ```
    #[must_use]
    pub fn get(&self, pos: Point) -> Option<T> {
        if pos.x < 0 || pos.y < 0 {
            return None;
        }
        let x = usize::try_from(pos.x).ok()?;
        let y = usize::try_from(pos.y).ok()?;
        if x >= self.width || y >= self.height {
            return None;
        }
        let idx = y * self.width + x;
        self.cells.get(idx).copied()
    }

    /// Sets the value at the given point.
    ///
    /// # Panics
    ///
    /// Panics if the point is out of bounds or negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_shared::{Grid, Point};
    /// let mut grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
    /// grid.set(Point::new(0, 0), 9);
    /// assert_eq!(grid.get(Point::new(0, 0)), Some(9));
    /// ```
    pub fn set(&mut self, pos: Point, value: T) {
        let x = usize::try_from(pos.x).expect("number too large");
        let y = usize::try_from(pos.y).expect("number too large");
        let idx = y * self.width + x;
        self.cells[idx] = value;
    }

    /// Returns `true` if the given point is within the grid bounds.
    ///
    /// # Panics
    ///
    /// Panics if `pos.x` or `pos.y` is negative or too large to convert to `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_shared::{Grid, Point};
    /// let grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
    /// assert!(grid.in_bounds(Point::new(1, 1)));
    /// assert!(!grid.in_bounds(Point::new(2, 2)));
    /// ```
    #[must_use]
    pub fn in_bounds(&self, pos: Point) -> bool {
        if pos.x < 0 || pos.y < 0 {
            return false;
        }
        let x = usize::try_from(pos.x).expect("number too large");
        let y = usize::try_from(pos.y).expect("number too large");
        (x) < self.width && (y) < self.height
    }

    /// Finds the first position of the given value in the grid.
    ///
    /// Returns `Some(Point)` if found, or `None` if not present.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_shared::{Grid, Point};
    /// let grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
    /// assert_eq!(grid.find_position(3), Some(Point::new(0, 1)));
    /// assert_eq!(grid.find_position(9), None);
    /// ```
    pub fn find_position(&self, target: T) -> Option<Point> {
        self.iter().find(|&(_, &c)| c == target).map(|(p, _)| p)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grid_from_vec() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let grid = Grid::from(input);

        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.cells, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_grid_from_empty_vec() {
        let input: Vec<Vec<i32>> = vec![];
        let grid = Grid::from(input);

        assert_eq!(grid.width, 0);
        assert_eq!(grid.height, 0);
        assert!(grid.cells.is_empty());
    }

    #[test]
    fn test_grid_from_iter() {
        let input = vec![vec![1, 2], vec![3, 4]];
        let grid: Grid<i32> = input.into_iter().collect();

        assert_eq!(grid.width, 2);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.cells, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_grid_get() {
        let grid = Grid {
            cells: vec![1, 2, 3, 4],
            height: 2,
            width: 2,
        };

        assert_eq!(grid.get(Point { x: 0, y: 0 }), Some(1));
        assert_eq!(grid.get(Point { x: 1, y: 0 }), Some(2));
        assert_eq!(grid.get(Point { x: 0, y: 1 }), Some(3));
        assert_eq!(grid.get(Point { x: 1, y: 1 }), Some(4));
    }

    #[test]
    fn test_grid_get_out_of_bounds() {
        let grid = Grid {
            cells: vec![1, 2, 3, 4],
            height: 2,
            width: 2,
        };

        assert_eq!(grid.get(Point { x: 2, y: 0 }), None);
        assert_eq!(grid.get(Point { x: 0, y: 2 }), None);
        assert_eq!(grid.get(Point { x: -1, y: 0 }), None);
        assert_eq!(grid.get(Point { x: 0, y: -1 }), None);
    }

    #[test]
    fn test_grid_set() {
        let mut grid = Grid {
            cells: vec![1, 2, 3, 4],
            height: 2,
            width: 2,
        };

        grid.set(Point { x: 0, y: 0 }, 5);
        assert_eq!(grid.get(Point { x: 0, y: 0 }), Some(5));

        grid.set(Point { x: 1, y: 1 }, 6);
        assert_eq!(grid.get(Point { x: 1, y: 1 }), Some(6));
    }

    #[test]
    #[should_panic(expected = "number too large")]
    fn test_grid_set_negative_x() {
        let mut grid = Grid {
            cells: vec![1, 2, 3, 4],
            height: 2,
            width: 2,
        };
        grid.set(Point { x: -1, y: 0 }, 5);
    }

    #[test]
    #[should_panic(expected = "number too large")]
    fn test_grid_set_negative_y() {
        let mut grid = Grid {
            cells: vec![1, 2, 3, 4],
            height: 2,
            width: 2,
        };
        grid.set(Point { x: 0, y: -1 }, 5);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 4 but the index is 6")]
    fn test_grid_set_out_of_bounds() {
        let mut grid = Grid {
            cells: vec![1, 2, 3, 4],
            height: 2,
            width: 2,
        };
        grid.set(Point { x: 2, y: 2 }, 5);
    }
}
