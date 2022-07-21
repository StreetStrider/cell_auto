
use super::Base;
use super::Point;
use super::super::Cell;


pub trait GridRead
{
	type Item;

	fn get (&self, point: &Point) -> Option<&Self::Item>;
	fn get_range (&self, points: Vec<Point>) -> Vec<(Point, Option<&Self::Item>)>;
}


pub struct Grid <Item: Cell, const Size: usize>
{
	pub table: [ [ Item; Size ]; Size ],
}


impl <Item: Cell, const Size: usize> GridRead for Grid<Item, Size>
{
	type Item = Item;

	fn get (&self, point: &Point) -> Option<&Item>
	{
		let (x, y) = self.ack(point)?;
		Some(&self.table[y][x])
	}

	fn get_range (&self, points: Vec<Point>) -> Vec<(Point, Option<&Item>)>
	{
		points.iter()
		.map(|point|
		{
			(*point, self.get(&point))
		})
		.collect()
	}
}


impl <Item: Cell, const Size: usize> Grid<Item, Size>
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


impl <'G, Item: Cell, const Size: usize> IntoIterator for &'G Grid<Item, Size>
{
	type Item = (Point, &'G Item);
	type IntoIter = GridIterator<'G, Item, Size>;

	fn into_iter (self) -> Self::IntoIter
	{
		GridIterator::new(self)
	}
}


pub struct GridIterator <'G, Item: Cell, const Size: usize>
{
	grid: &'G Grid<Item, Size>,
	x: usize,
	y: usize,
}

impl <'G, Item: Cell, const Size: usize> GridIterator<'G, Item, Size>
{
	fn new (grid: &'G Grid<Item, Size>) -> Self
	{
		GridIterator { grid, x: 0, y: 0 }
	}
}

impl <'G, Item: Cell, const Size: usize> Iterator for GridIterator<'G, Item, Size>
{
	type Item = (Point, &'G Item);

	fn next (&mut self) -> Option<Self::Item>
	{
		let Self { grid, mut x, mut y } = self;

		if y == Size { return None }

		let pt = Point::new(x as Base, y as Base);
		let next = &grid.table[y][x];

		if (x == Size - 1)
		{
			x = 0;
			y = (y + 1);
		}
		else
		{
			x = (x + 1);
		}

		(self.x, self.y) = (x, y);

		Some((pt, next))
	}
}
