
use super::Point;


pub trait Cellular
{
	fn new () -> Self;
}


pub struct Grid <Item: Cellular + Copy, const Size: usize>
{
	pub table: [ [ Item; Size ]; Size ],
}


impl <Item: Cellular + Copy, const Size: usize> Grid<Item, Size>
{
	pub fn new () -> Self
	{
		Grid { table: [ [ Item::new(); Size ]; Size ] }
	}

	pub fn get (&self, point: &Point) -> Item
	{
		let (x, y) = point.to_usize();
		self.table[y][x]
	}

	pub fn set (&mut self, point: &Point, item: Item)
	{
		let (x, y) = point.to_usize();
		self.table[y][x] = item;
	}
}
