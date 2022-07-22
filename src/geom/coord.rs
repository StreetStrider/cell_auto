
use crate::TermScalar;

use super::Base;
use super::Scalar;
use super::Offset;


#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(Debug)]
pub struct Coord(pub Base);


impl Scalar for Coord
{
	fn new (value: Base) -> Self
	{
		Self(value)
	}
}


//
// From
//
impl From<Coord> for TermScalar
{
	fn from (coord: Coord) -> TermScalar
	{
		TermScalar::try_from(coord.0).unwrap()
	}
}

impl From<Coord> for usize
{
	fn from (coord: Coord) -> usize
	{
		coord.0 as usize
	}
}

impl From<usize> for Coord
{
	fn from (size: usize) -> Coord
	{
		Coord::new(Base::try_from(size).unwrap())
	}
}


//
// Coord + Offset = Coord
//
impl std::ops::Add<Offset> for Coord
{
	type Output = Coord;

	fn add (self, offset: Offset) -> Self
	{
		Coord(self.0 + offset.0)
	}
}

//
// Coord - Coord = Offset
//
impl std::ops::Sub for Coord
{
	type Output = Offset;

	fn sub (self, R: Coord) -> Offset
	{
		Offset(self.0 - R.0)
	}
}

//
// -Coord = Coord
//
impl std::ops::Neg for Coord
{
	type Output = Coord;

	fn neg (self) -> Self
	{
		Coord(-self.0)
	}
}
