#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

use termion;

mod geom;

use geom::Base;
use geom::Offset;

use geom::Point;
use geom::Arrow;

// use geom::Area;

use geom::grid::Grid;
use geom::grid::Cellular;


type TermScalar = u16;

const termion_goto_arrow: Arrow = Arrow { x: Offset(1), y: Offset(1) };


fn main ()
{
	let mut view = View::new();

	view.grid.set(&Point::new(0, 0), Cell::Fill);
	view.grid.set(&Point::new(0, 39), Cell::Fill);
	view.grid.set(&Point::new(199, 0), Cell::Fill);
	view.grid.set(&Point::new(199, 39), Cell::Fill);

	view.grid.set(&Point::new(4, 3), Cell::Fill);
	view.grid.set(&Point::new(5, 5), Cell::Fill);
	view.grid.set(&Point::new(5, 6), Cell::Fill);
	view.grid.set(&Point::new(6, 5), Cell::Fill);
	view.grid.set(&Point::new(105, 45), Cell::Fill);
	view.grid.set(&Point::new(106, 45), Cell::Fill);
	view.grid.set(&Point::new(107, 45), Cell::Fill);
	view.grid.set(&Point::new(108, 45), Cell::Fill);
	view.grid.set(&Point::new(198, 45), Cell::Fill);
	view.grid.set(&Point::new(199, 45), Cell::Fill);
	view.grid.set(&Point::new(199, 199), Cell::Fill);

	view.draw();

	loop {}
}


#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
enum Cell
{
	Empty,
	Fill,
}

impl Cellular for Cell
{
	fn empty () -> Self
	{
		Cell::Empty
	}
}


struct View
{
	grid: Grid<Cell, 200>,
	root: Arrow,
	// viewport: Area,
	camera: Arrow,
}

impl View
{
	fn new () -> View
	{
		let root = Arrow::new(0, 1);
		// let extent = Arrow::new(30, 20);
		let camera = Arrow::new(0, 0);
		// let camera = Arrow::new(4, 3);

		View
		{
			grid: Grid::new(),
			root,
			// viewport: Area { root, extent },
			camera,
		}
	}

	fn draw (&self)
	{
		self.clear();

		let view_size = terminal_size() - self.root - Arrow::new(0, 5);
		let (rows, cols) = view_size.to_range();
		print!("{:?};{:?}", cols, rows);

		let term_root = (Point::zero() + self.root + termion_goto_arrow);
		let grid_root = (Point::zero() + self.camera);

		for row_n in rows
		{
			for col_n in cols.clone()
			{
				let a_rel = Arrow::new(col_n, row_n);
				let p_term = (term_root + a_rel);
				let p_grid = (grid_root + a_rel);

				print!("{}", p_term.to_cursor());

				let cell = self.grid.get(&p_grid);

				if let Some(cell) = cell
				{
					print!("{}", match cell
					{
						Cell::Empty => '·',
						Cell::Fill  => '█',
					});
				}
			}
		}
		print!("\n");
	}

	fn clear (&self)
	{
		print!("{}", termion::clear::All);
		print!("{}", termion::cursor::Goto(1, 1));
	}
}

fn terminal_size () -> Arrow
{
	let (x, y) = termion::terminal_size().unwrap();

	Arrow::new(x as Base, y as Base)
}
