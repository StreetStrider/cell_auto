#![allow(unused_parens)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
// #![allow(dead_code)]
// #![allow(non_camel_case_types)]

use std::io::{stdin, stdout};

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use termion::raw::IntoRawMode;
use termion::cursor::HideCursor;
use termion::input::TermRead;

use tempus_fugit::measure;

mod geom;

use geom::Point;
use geom::Arrow;

mod cell;
use cell::Cell;

use geom::grid::Grid;
use geom::grid::Square;
// use geom::grid::Torus;

mod doublegrid;
use doublegrid::DoubleGrid;

mod view;
use view::View;

type TermScalar = u16;

const size: usize = 100;
const delay: u64 = 32;

pub type G1 = DoubleGrid<Square<C1, size>>;
// pub type G1 = DoubleGrid<Torus<C1, size>>;

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
	let stdout = stdout().into_raw_mode().unwrap();
	let _hide = HideCursor::from(stdout);

	let finished = Arc::new(Mutex::new(false));
	let paused   = Arc::new(Mutex::new(false));

	thread::spawn(th_input(
		Arc::clone(&finished),
		Arc::clone(&paused),
	));

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

	view.clear();

	loop
	{
		if *finished.lock().unwrap() { break }

		let run_cycle = ! *paused.lock().unwrap();
		let run_draw  = true;

		if run_cycle
		{
			view.tick();
		}

		if run_draw
		{
			let (_, measurement) = measure!
			{
				view.draw(&*dugrid.get())
			};
			view.m_draw = measurement;
		}

		thread::sleep(Duration::from_millis(delay));

		if run_cycle
		{
			let (_, measurement) = measure!
			{
				cycle(&moore, &mut dugrid)
			};

			view.m_cycle = measurement;
		}
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

fn cycle_each <C: Cell, F: Fn(&Point, &C) -> C> (src: &impl Grid<Cell = C>, dst: &mut impl Grid<Cell = C>, fn_map: F)
{
	src.each(|point, cell|
	{
		let cell_next = fn_map(point, cell);

		dst.set(point, cell_next);
	})
}

fn fill_moore_of <C: Cell> (moore: &Moore, grid: &impl Grid<Cell = C>, point: &Point) -> usize
{
	let mut sum = 0usize;

	for arrow in moore
	{
		match grid.get(&(*point + *arrow))
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

fn th_input (
	finished: Arc<Mutex<bool>>,
	paused:   Arc<Mutex<bool>>,
)
-> impl FnOnce() -> ()
{
	move ||
	{
		use termion::event::Key;

		for input in stdin().keys()
		{
			match input.unwrap()
			{
				Key::Char('q') =>
				{
					*finished.lock().unwrap() = true;
				},
				Key::Char(' ') =>
				{
					let mut paused = paused.lock().unwrap();
					*paused = ! *paused;
				}
				_ => {},
			}
		}
	}
}
