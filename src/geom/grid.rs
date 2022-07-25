
use super::Point;

pub mod square;
pub use square::Square;

pub mod torus;
pub use torus::Torus;


pub trait Grid
{
	type Cell;

	fn new () -> Self;
	fn get (&self, point: &Point) -> Option<&Self::Cell>;
	fn each <F: FnMut(&Point, &Self::Cell) -> ()> (&self, fn_each: F) -> ();
	fn set (&mut self, point: &Point, cell: Self::Cell) -> Option<()>;
}
