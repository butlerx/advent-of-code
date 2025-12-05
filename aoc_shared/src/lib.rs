#![warn(clippy::pedantic, clippy::perf)]

//! Shared utilities for Advent of Code solutions.
//!
//! This crate provides reusable types and helpers for Advent of Code puzzles, including:
//! - [`Point`]: 2D integer coordinates with neighbor and distance utilities
//! - [`Grid`]: 2D grid structure for storing and manipulating values
//! - Timer utilities for benchmarking code
//!
//! # Examples
//!
//! ```
//! use aoc_shared::{Grid, Point};
//! let grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
//! let p = Point::new(1, 0);
//! assert_eq!(grid.get(p), Some(2));
//! ```

mod grid;
mod point;
mod timer;

pub use grid::Grid;
pub use point::Point;
pub use timer::*;
