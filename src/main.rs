#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

mod geom;

use geom::Point;

mod cell;
use cell::Cell;

mod doublegrid;
use doublegrid::DoubleGrid;

mod view;
use view::View;

type TermScalar = u16;

// pub type G1 = Grid<C1, 200>;
pub type G2 = DoubleGrid<C1, 200>;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub enum C1
{
	Empty,
	Fill,
}

impl Cell for C1
{
	fn empty () -> Self
	{
		Self::Empty
	}

	fn draw (&self) -> char
	{
		match self
		{
			Self::Empty => '·',
			Self::Fill  => '█',
		}
	}
}


fn main ()
{
	let mut dugrid = G2::new();
	let grid = dugrid.get_next();

	grid.set(&Point::new(0, 0), C1::Fill);
	grid.set(&Point::new(0, 39), C1::Fill);
	grid.set(&Point::new(199, 0), C1::Fill);
	grid.set(&Point::new(199, 39), C1::Fill);

	grid.set(&Point::new(4, 3), C1::Fill);
	grid.set(&Point::new(5, 5), C1::Fill);
	grid.set(&Point::new(5, 6), C1::Fill);
	grid.set(&Point::new(6, 5), C1::Fill);
	grid.set(&Point::new(105, 45), C1::Fill);
	grid.set(&Point::new(106, 45), C1::Fill);
	grid.set(&Point::new(107, 45), C1::Fill);
	grid.set(&Point::new(108, 45), C1::Fill);
	grid.set(&Point::new(198, 45), C1::Fill);
	grid.set(&Point::new(199, 45), C1::Fill);
	grid.set(&Point::new(199, 199), C1::Fill);

	dugrid.switch();
	let view = View::<C1>::new(dugrid.get());
	view.draw();

	let grid = dugrid.get_next();
	grid.set(&Point::new(0, 0), C1::Fill);

	dugrid.switch();
	let view = View::<C1>::new(dugrid.get());
	view.draw();

	loop {}
}
