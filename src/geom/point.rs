
use termion::cursor::Goto;

use super::Pair;
use super::Coord;
use super::Arrow;

pub type Point = Pair<Coord>;


//
// From
//
impl From<Point> for Goto
{
	fn from (point: Point) -> Goto
	{
		let Point { x, y } = point;
		Goto(x.into(), y.into())
	}
}

impl From<(usize, usize)> for Point
{
	fn from ((x, y): (usize, usize)) -> Point
	{
		Point { x: x.into(), y: y.into() }
	}
}

impl From<Point> for (usize, usize)
{
	fn from (point: Point) -> (usize, usize)
	{
		(point.x.into(), point.y.into())
	}
}


//
// Point + Arrow = Point
//
impl std::ops::Add<Arrow> for Point
{
	type Output = Self;

	#[inline]
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
