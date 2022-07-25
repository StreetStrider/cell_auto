
use super::super::Point;
use super::super::super::Cell;

use super::Grid;


pub struct Square <C: Cell, const Size: usize>
{
	pub table: Box<[C]>,
}


impl <C: Cell, const Size: usize> Square<C, Size>
{
	const Size2: usize = (Size * Size);

	#[inline]
	fn ack (&self, point: &Point) -> Option<usize>
	{
		let (x, y): (usize, usize) = (*point).into();

		if x >= Size { return None }
		if y >= Size { return None }

		Some(y * Size + x)
	}
}

impl <C: Cell, const Size: usize> Grid for Square<C, Size>
{
	type Cell = C;

	fn new () -> Self
	{
		let empty = Cell::empty();

		let mut table = Vec::with_capacity(Self::Size2);
		for _ in 0..Self::Size2
		{
			table.push(empty);
		}

		let table = table.into_boxed_slice();

		Self { table }
	}

	#[inline]
	fn get (&self, point: &Point) -> Option<&Self::Cell>
	{
		let index = self.ack(point)?;
		Some(&self.table[index])
	}

	#[inline]
	fn set (&mut self, point: &Point, cell: C) -> Option<()>
	{
		let index = self.ack(point)?;
		self.table[index] = cell;
		Some(())
	}

	fn each <F: FnMut(&Point, &Self::Cell) -> ()> (&self, mut fn_each: F) -> ()
	{
		let mut point = Point::zero();

		for index in 0..Self::Size2
		{
			point.y = (index / Size).into();
			point.x = (index % Size).into();

			let next = &self.table[index];

			fn_each(&point, &next);
		}
	}
}
