
use std::cell::Ref;
use std::cell::RefMut;
use std::cell::RefCell;

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


pub struct DoubleGrid <Item: Cell, const Size: usize>
{
	current: Current,
	one: RefCell<Grid<Item, Size>>,
	two: RefCell<Grid<Item, Size>>,
}


impl <Item: Cell, const Size: usize> DoubleGrid<Item, Size>
{
	pub fn new () -> Self
	{
		let current = Current::One;
		let one = RefCell::new(Grid::<Item, Size>::new());
		let two = RefCell::new(Grid::<Item, Size>::new());

		DoubleGrid { current, one, two }
	}

	pub fn get (&self) -> Ref<'_, Grid<Item, Size>>
	{
		match self.current
		{
			One => self.one.borrow(),
			Two => self.two.borrow(),
		}
	}

	pub fn get_next (&self) -> RefMut<'_, Grid<Item, Size>>
	{
		match self.current
		{
			One => self.two.borrow_mut(),
			Two => self.one.borrow_mut(),
		}
	}

	pub fn switch (&mut self)
	{
		self.current = self.current.switch();
	}
}
