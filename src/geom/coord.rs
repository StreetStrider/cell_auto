
use super::Base;
use super::Scalar;
use super::Offset;
use crate::TermScalar;


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


impl Coord
{
	pub fn new (value: Base) -> Coord
	{
		Coord(value)
	}

	pub fn to_cursor (self) -> TermScalar
	{
		TermScalar::try_from(self.0).unwrap()
	}
}
