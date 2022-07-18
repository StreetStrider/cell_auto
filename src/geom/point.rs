
use termion::cursor::Goto;

use super::Pair;
use super::Coord;
use super::Arrow;

pub type Point = Pair<Coord>;


//
// Point + Arrow = Point
//
impl std::ops::Add<Arrow> for Point
{
	type Output = Self;

	fn add (self, arrow: Arrow) -> Self
	{
		Point::new(self.x.0 + arrow.x.0, self.y.0 + arrow.y.0)
	}
}

//
// Point - Arrow = Point
//
impl std::ops::Sub<Arrow> for Point
{
	type Output = Self;

	fn sub (self, arrow: Arrow) -> Self
	{
		Point::new(self.x.0 - arrow.x.0, self.y.0 - arrow.y.0)
	}
}

//
// -Point = Point
//
impl std::ops::Neg for Point
{
	type Output = Self;

	fn neg (self) -> Self
	{
		Point::new(-self.x.0, -self.y.0)
	}
}


impl Point
{
	pub fn to_cursor (&self) -> Goto
	{
		Goto(self.x.to_cursor(), self.y.to_cursor())
	}
}
