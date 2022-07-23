#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

use tempus_fugit::measure;

use std::thread::sleep;
use std::time::Duration;

mod geom;

use geom::Point;
use geom::Arrow;

mod cell;
use cell::Cell;

use geom::grid::Grid;
use geom::grid::GridRead;

mod doublegrid;
use doublegrid::DoubleGrid;

mod view;
use view::View;

type TermScalar = u16;

// const size: usize = 1000;
const size: usize = 30;
const delay: u64 = 32;

// pub type G1 = Grid<C1, size>;
pub type G1 = DoubleGrid<C1, size>;

#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq, Eq)]
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

type Moore = [Arrow; 8];


fn main ()
{
	let moore = moore();

	let mut dugrid = G1::new();
	let mut view = View::new();

	{
		let mut grid = dugrid.get_next();

		grid.set(&Point::new(3, 3), Fill);
		grid.set(&Point::new(3, 4), Fill);
		grid.set(&Point::new(3, 5), Fill);

		grid.set(&Point::new(9, 3), Fill);
		grid.set(&Point::new(9, 4), Fill);
		grid.set(&Point::new(9, 5), Fill);
		grid.set(&Point::new(8, 5), Fill);
		grid.set(&Point::new(7, 4), Fill);
	}

	dugrid.switch();

	loop
	{
		view.tick();

		let (_, measurement) = measure!
		{
			view.draw(&*dugrid.get())
		};
		view.m_draw = measurement;

		sleep(Duration::from_millis(delay));

		let (_, measurement) = measure!
		{
			cycle(&moore, &mut dugrid)
		};

		view.m_cycle = measurement;
	}
}


fn cycle (moore: &Moore, dugrid: &mut G1)
{
	{
		let src = &*dugrid.get();
		let dst = &mut *dugrid.get_next();
		cycle_each(src, dst, |pt, cell|
		{
			let nebs = fill_moore_of(moore, src, &pt);

			match cell
			{
				Empty => match nebs
				{
					3 => Fill,
					_ => Empty,
				},
				Fill  => match nebs
				{
					2 | 3 => Fill,
					_ => Empty,
				},
			}
		});
	}

	dugrid.switch();
}

fn cycle_each <Cell: cell::Cell, const Size: usize, F: Fn(&Point, &Cell) -> Cell> (src: &Grid<Cell, Size>, dst: &mut Grid<Cell, Size>, fn_map: F)
{
	src.each(|point, cell|
	{
		let cell_next = fn_map(point, cell);

		dst.set(point, cell_next);
	})
}

fn fill_moore_of <Cell: cell::Cell, const Size: usize> (moore: &Moore, grid: &Grid<Cell, Size>, point: &Point) -> usize
{
	let mut sum = 0usize;

	for arrow in *moore
	{
		match grid.get(&(*point + arrow))
		{
			Some(&cell) if cell != Cell::empty() => { sum = (sum + 1); }
			_ => {},
		}
	}

	sum
}

fn moore () -> Moore
{
	let range = [-1, 0, 1];

	let mut moore = [Arrow::zero(); 8];
	let mut next = 0;

	for x in range.clone()
	{
		for y in range.clone()
		{
			match (x, y)
			{
				(0, 0) => {},
				(_, _) =>
				{
					moore[next] = Arrow::new(x, y);
					next = (next + 1);
				}
			}
		}
	}

	moore
}
