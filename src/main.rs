#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

use std::thread::sleep;
use std::time::Duration;

mod geom;

use geom::Point;

mod cell;
use cell::Cell;

// use geom::grid::GridRead;

mod doublegrid;
use doublegrid::DoubleGrid;

mod view;
use view::View;

type TermScalar = u16;

const size: usize = 200;

// pub type G1 = Grid<C1, size>;
pub type G1 = DoubleGrid<C1, size>;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub enum C1
{
	Empty,
	Fill,
}

use C1::*;

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
	let mut dugrid = G1::new();
	let mut view = View::new();

	{
		let mut grid = dugrid.get_next();

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
	}

	dugrid.switch();

	loop
	{
		cycle(&mut dugrid);

		view.tick();
		view.draw(&*dugrid.get());

		sleep(Duration::from_millis(500));
	}
}

fn cycle (dugrid: &mut G1)
{
	{
		let grid = dugrid.get();

		for (pt, cell) in &*grid
		{
			let cell_next = match cell
			{
				Empty => Fill,
				Fill  => Empty,
			};

			let mut grid_next = dugrid.get_next();
			grid_next.set(&pt, cell_next);
		}
	}

	dugrid.switch();
}
