
use std::fmt;

use super::Base;
use super::Scalar;
use super::Coord;


#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(Debug)]
pub struct Pair <T: Scalar = Coord>
{
	pub x: T,
	pub y: T,
}


impl <T: Scalar> Pair<T>
{
	pub fn new (x: Base, y: Base) -> Self
	{
		Self { x: T::new(x), y: T::new(y) }
	}

	pub fn zero () -> Self
	{
		Self::new(0, 0)
	}

	pub fn unit () -> Self
	{
		Self::new(1, 1)
	}
}


impl <T: Scalar> fmt::Display for Pair<T>
{
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "({},{})", self.x, self.y)
	}
}
