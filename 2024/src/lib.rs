#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

mod grid;
mod point;
mod timer;

pub use grid::Grid;
pub use point::Point;
pub use timer::time_execution;
