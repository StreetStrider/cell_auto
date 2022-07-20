
use super::cell::Cell;
use super::geom::grid::Grid;


pub enum Current
{
	One,
	Two,
}

use Current::*;

impl Current
{
	pub fn switch (&self) -> Self
	{
		match self
		{
			One => Two,
			Two => One,
		}
	}
}


pub struct DoubleGrid <Item: Cell + Copy, const Size: usize>
{
	current: Current,
	one: Box<Grid<Item, Size>>,
	two: Box<Grid<Item, Size>>,
}


impl <Item: Cell + Copy, const Size: usize> DoubleGrid<Item, Size>
{
	pub fn new () -> Self
	{
		let current = Current::One;
		let one = Box::new(Grid::<Item, Size>::new());
		let two = Box::new(Grid::<Item, Size>::new());

		DoubleGrid { current, one, two }
	}

	pub fn get (&self) -> &Grid<Item, Size>
	{
		match self.current
		{
			One => & *self.one,
			Two => & *self.two,
		}
	}

	pub fn get_next (&mut self) -> &mut Grid<Item, Size>
	{
		match self.current
		{
			One => &mut *self.two,
			Two => &mut *self.one,
		}
	}

	pub fn switch (&mut self)
	{
		self.current = self.current.switch();
	}
}
