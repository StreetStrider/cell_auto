// pub trait Scalar: Clone + Copy + PartialEq + Eq + PartialOrd + Ord {}
// https://stackoverflow.com/questions/56450533/is-there-a-way-to-alias-multiple-derives-as-a-single-one
// not yet

use std::fmt::Display;

pub type Base = i32;

pub trait Scalar: Display + Copy + Eq + Ord
{
	fn new (value: Base) -> Self;
}

pub mod coord;
pub use coord::Coord;

pub mod offset;
pub use offset::Offset;

pub mod pair;
pub use pair::Pair;

pub mod point;
pub use point::Point;

pub mod arrow;
pub use arrow::Arrow;

pub mod area;
pub use area::Area;

pub mod grid;
pub use grid::Grid;
