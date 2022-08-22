#![allow(unused_parens)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
// #![allow(dead_code)]
// #![allow(non_camel_case_types)]

use std::io::{stdin, stdout};

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::time::Duration;

use num_cpus::get as cpus;

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

mod moore;
use moore::Moore;

mod range_sect;

type TermScalar = u16;

const size: usize = 7120;
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

fn main ()
{
	// let stdout = stdout().into_raw_mode().unwrap();
	// let _hide = HideCursor::from(stdout);

	let cpus = cpus();
	let buckets = range_sect::sect(size, cpus);
	// let sect = sect(10, cpus);
	println!("{:?}", buckets);
	// println!("{:?}", sect(16, cpus));
	// println!("{:?}", sect(32, cpus));

	std::process::exit(0);

	let finished = Arc::new(Mutex::new(false));
	let paused   = Arc::new(Mutex::new(false));

	let moore = moore::new();

	let dugrid = G1::new();
	let dugrid = Arc::new(Mutex::new(dugrid));

	let view = View::new();
	let view = Arc::new(Mutex::new(view));

	{
		let grid = dugrid.lock().unwrap();
		let mut grid = grid.get_next();

		grid.set(&Point::new(3, 3), Fill);
		grid.set(&Point::new(3, 4), Fill);
		grid.set(&Point::new(3, 5), Fill);

		grid.set(&Point::new(9, 3), Fill);
		grid.set(&Point::new(9, 4), Fill);
		grid.set(&Point::new(9, 5), Fill);
		grid.set(&Point::new(8, 5), Fill);
		grid.set(&Point::new(7, 4), Fill);
	}

	dugrid.lock().unwrap().switch();

	view.lock().unwrap().clear();

	thread::spawn(th_draw(
		Arc::clone(&view),
		Arc::clone(&dugrid),
	));

	// thread::sleep(Duration::from_millis(delay / 2));

	thread::spawn(th_input(
		Arc::clone(&finished),
		Arc::clone(&paused),
	));

	loop
	{
		if *finished.lock().unwrap() { break }

		thread::sleep(Duration::from_millis(delay));

		if ! *paused.lock().unwrap()
		{
			let mut view = view.lock().unwrap();

			view.tick();

			let (_, measurement) = measure!
			{
				cycle(&moore, &mut dugrid.lock().unwrap())
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
			let nebs = moore::fill(moore, src, &pt);

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
	src.each(|_, point, cell|
	{
		let cell_next = fn_map(point, cell);

		dst.set(point, cell_next);
	})
}

fn th_draw (
	view: Arc<Mutex<View>>,
	dugrid: Arc<Mutex<G1>>,
)
-> impl FnOnce() -> ()
{
	move ||
	{
		let run_draw = true;

		while run_draw
		{
			{
				let dugrid = dugrid.lock().unwrap();
				let grid = &*dugrid.get();
				let mut view = view.lock().unwrap();

				let (_, measurement) = measure!
				{
					view.draw(grid)
				};
				view.m_draw = measurement;
			}

			thread::sleep(Duration::from_millis(delay));
		}
	}
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
