
use std::ops::Range;

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
	const range_total: Range<usize> = 0..Self::Size2;

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
	const Size: usize = Size;
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

	#[inline]
	fn each_range <F> (&self, range: Range<usize>, mut fn_each: F) -> ()
		where F: FnMut(usize, &Point, &Self::Cell) -> ()
	{
		#[cfg(debug_assertions)]
		assert!(range_contains_range(&Self::range_total, &range));

		let mut point = Point::zero();

		for index in range
		{
			point.y = (index / Size).into();
			point.x = (index % Size).into();

			let next = &self.table[index];

			fn_each(index, &point, &next);
		}
	}

	#[inline]
	fn each <F> (&self, fn_each: F) -> ()
		where F: FnMut(usize, &Point, &Self::Cell) -> ()
	{
		self.each_range(Self::range_total, fn_each)
	}
}


#[cfg(debug_assertions)]
fn range_contains_range <T> (range: &Range<T>, subrange: &Range<T>) -> bool
	where T: PartialOrd
{
	if subrange.start < range.start { return false }
	if   subrange.end > range.end   { return false }
	true
}
