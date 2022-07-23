
use super::Point;
// use super::super::Cell;


pub trait GridRead
{
	type Cell;

	fn get (&self, point: &Point) -> Option<&Self::Cell>;
	fn each <F: FnMut(&Point, &Self::Cell) -> ()> (&self, fn_each: F) -> ();
}


pub struct Grid <Cell: super::super::Cell, const Size: usize>
{
	pub table: [ [ Cell; Size ]; Size ],
}


impl <Cell: super::super::Cell, const Size: usize> GridRead for Grid<Cell, Size>
{
	type Cell = Cell;

	#[inline]
	fn get (&self, point: &Point) -> Option<&Cell>
	{
		let (x, y) = self.ack(point)?;
		Some(&self.table[y][x])
	}

	fn each <F: FnMut(&Point, &Self::Cell) -> ()> (&self, mut fn_each: F) -> ()
	{
		let mut point = Point::zero();

		for y in 0..Size
		{
			for x in 0..Size
			{
				point.x = x.into();
				point.y = y.into();

				let next = &self.table[y][x];

				fn_each(&point, &next);
			}
		}
	}
}


impl <Cell: super::super::Cell, const Size: usize> Grid<Cell, Size>
{
	pub fn new () -> Self
	{
		Grid { table: [ [ Cell::empty(); Size ]; Size ] }
	}

	#[inline]
	pub fn ack (&self, point: &Point) -> Option<(usize, usize)>
	{
		let (x, y): (usize, usize) = (*point).into();

		if x >= Size { return None }
		if y >= Size { return None }

		Some((x, y))
	}

	#[inline]
	pub fn set (&mut self, point: &Point, cell: Cell) -> Option<()>
	{
		let (x, y) = self.ack(point)?;
		self.table[y][x] = cell;
		Some(())
	}
}


/*
impl <'G, Item: Cell, const Size: usize> IntoIterator for &'G Grid<Item, Size>
{
	type Item = (&Point, &'G Item);
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
	point: Point,
}

impl <'G, Item: Cell, const Size: usize> GridIterator<'G, Item, Size>
{
	fn new (grid: &'G Grid<Item, Size>) -> Self
	{
		GridIterator { grid, x: 0, y: 0, point: Point::zero() }
	}
}

impl <'G, Item: Cell, const Size: usize> Iterator for GridIterator<'G, Item, Size>
{
	type Item = (&Point, &'G Item);

	fn next (&mut self) -> Option<Self::Item>
	{
		let Self { grid, mut x, mut y, mut point } = self;

		if y == Size { return None }

		point.x = x.into();
		point.y = y.into();

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

		Some((&self.point, next))
	}
}
*/
