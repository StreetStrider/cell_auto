// TODO: rm render reflections

use super::super::Point;
use super::super::super::Cell;

use super::Grid;
use super::Square;


pub struct Torus <C: Cell, const Size: usize>
{
	pub square: Square<C, Size>,
}


impl <C: Cell, const Size: usize> Torus<C, Size>
{
	#[inline]
	fn ack (&self, point: &Point) -> (usize, usize)
	{
		let (x, y): (usize, usize) = (*point).into();

		let x = ((x % Size) + Size) % Size;
		let y = ((y % Size) + Size) % Size;

		(x, y)
	}
}


impl <C: Cell, const Size: usize> Grid for Torus<C, Size>
{
	const Size: usize = Size;
	type Cell = C;

	fn new () -> Self
	{
		Torus { square: Square::<C, Size>::new() }
	}

	#[inline]
	fn get (&self, point: &Point) -> Option<&Self::Cell>
	{
		let pair = self.ack(point);
		self.square.get(&(pair.into()))
	}

	#[inline]
	fn set (&mut self, point: &Point, cell: C) -> Option<()>
	{
		self.square.set(point, cell)
	}

	#[inline]
	fn each <F> (&self, fn_each: F) -> ()
		where F: FnMut(usize, &Point, &Self::Cell) -> ()
	{
		self.square.each(fn_each)
	}
}
