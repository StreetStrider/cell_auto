
use super::Point;
use super::super::Cell;


pub trait GridRead
{
	type Item;

	fn get (&self, point: &Point) -> Option<&Self::Item>;
}


pub struct Grid <Item: Cell + Copy, const Size: usize>
{
	pub table: [ [ Item; Size ]; Size ],
}


impl <Item: Cell + Copy, const Size: usize> GridRead for Grid<Item, Size>
{
	type Item = Item;

	fn get (&self, point: &Point) -> Option<&Item>
	{
		let (x, y) = self.ack(point)?;
		Some(&self.table[y][x])
	}
}


impl <Item: Cell + Copy, const Size: usize> Grid<Item, Size>
{
	pub fn new () -> Self
	{
		Grid { table: [ [ Item::empty(); Size ]; Size ] }
	}

	pub fn ack (&self, point: &Point) -> Option<(usize, usize)>
	{
		let (x, y) = point.to_usize();

		if x >= Size { return None }
		if y >= Size { return None }

		Some((x, y))
	}

	pub fn set (&mut self, point: &Point, item: Item) -> Option<()>
	{
		let (x, y) = self.ack(point)?;
		self.table[y][x] = item;
		Some(())
	}
}
