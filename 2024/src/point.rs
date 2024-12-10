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
