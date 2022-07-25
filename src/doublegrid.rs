
use std::cell::RefCell;
use std::cell::{Ref, RefMut};

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


pub struct DoubleGrid <G: Grid>
{
	current: Current,
	one: RefCell<G>,
	two: RefCell<G>,
}


impl <G: Grid> DoubleGrid<G>
{
	pub fn new () -> Self
	{
		let current = Current::One;
		let one = RefCell::new(G::new());
		let two = RefCell::new(G::new());

		DoubleGrid { current, one, two }
	}

	pub fn get (&self) -> Ref<'_, G>
	{
		match self.current
		{
			One => self.one.borrow(),
			Two => self.two.borrow(),
		}
	}

	pub fn get_next (&self) -> RefMut<'_, G>
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
