#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

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

const size: usize = 200;
const delay: u64 = 2;

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


fn main ()
{
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
		view.draw(&*dugrid.get());

		sleep(Duration::from_millis(delay));

		cycle(&mut dugrid);
	}
}

fn cycle (dugrid: &mut G1)
{
	{
		let src = &*dugrid.get();
		let dst = &mut *dugrid.get_next();
		cycle_each(src, dst, |pt, cell|
		{
			let nebs = fill_moore_of(src, &pt);

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

fn cycle_each <Item: Cell, const Size: usize, F: Fn(Point, &Item) -> Item> (src: &Grid<Item, Size>, dst: &mut Grid<Item, Size>, fn_map: F)
{
	for (pt, cell) in src
	{
		let cell_next = fn_map(pt, cell);

		dst.set(&pt, cell_next);
	}
}

/*
fill_moore_of(&*grid, &Point::new(5, 5))


	let grid = dugrid.get();
	// println!("{:?}", moore_of(&*grid, &Point::new(1, 1)));
	for item in moore_of(&*grid, &Point::new(5, 5))
	{
		println!("{:?}", item);
	}
	// println!("{}", );
	return;
*/

fn fill_moore_of <'L, Item: Cell, const Size: usize> (grid: &'L Grid<Item, Size>, point: &Point) -> usize
{
	grid.get_range(moore(point))
	.iter()
	.filter(|(_, item)|
	{
		match item
		{
			Some(&item) if item != Item::empty() => true,
			_ => false,
		}
	})
	.count()
}

fn moore_of <'L, Item: Cell, const Size: usize> (grid: &'L Grid<Item, Size>, point: &Point)
 -> Vec<(Point, Option<&'L Item>)>
{
	grid.get_range(moore(point))
}

fn moore (point: &Point) -> Vec<Point>
{
	let range = [-1, 0, 1];
	let mut moore = vec![];

	for x in range.clone()
	{
		for y in range.clone()
		{
			match (x, y)
			{
				(0, 0) => (),
				(_, _) => moore.push(*point + Arrow::new(x, y)),
			}
		}
	}

	moore
}
