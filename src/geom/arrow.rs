
use std::ops::Range;

use crate::TermScalar;

use super::Base;

// use super::Coord;
use super::Offset;

use super::Pair;
use super::Point;

use super::Area;

pub type Arrow = Pair<Offset>;


//
// From
//
impl From<(TermScalar, TermScalar)> for Arrow
{
	fn from ((x, y): (TermScalar, TermScalar)) -> Arrow
	{
		Arrow::new(x.into(), y.into())
	}
}


//
// Arrow + Arrow = Arrow
//
impl std::ops::Add<Arrow> for Arrow
{
	type Output = Self;

	fn add (self, arrow: Arrow) -> Self
	{
		Arrow::new(self.x.0 + arrow.x.0, self.y.0 + arrow.y.0)
	}
}

//
// Arrow - Arrow = Arrow
//
impl std::ops::Sub<Arrow> for Arrow
{
	type Output = Self;

	fn sub (self, arrow: Arrow) -> Self
	{
		Arrow::new(self.x.0 - arrow.x.0, self.y.0 - arrow.y.0)
	}
}

//
// -Arrow = Arrow
//
impl std::ops::Neg for Arrow
{
	type Output = Self;

	fn neg (self) -> Self
	{
		Arrow::new(-self.x.0, -self.y.0)
	}
}


impl Arrow
{
	pub fn to_area (&self) -> Area
	{
		let root = Point::zero();
		let extent = self.clone();

		Area { root, extent }
	}

	pub fn to_range (&self) -> (Range<Base>, Range<Base>)
	{
		self.to_area().to_range()
	}
}
