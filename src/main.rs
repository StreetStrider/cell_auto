#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use termion;

mod geom;

use geom::Base;
// use geom::Offset;

use geom::Point;
use geom::Arrow;

use geom::Area;

use geom::grid::Grid;
use geom::grid::Cellular;


type TermScalar = u16;


fn main ()
{
	let mut view = View::new();

	view.grid.set(&Point::new(4, 3), Cell::Fill);
	view.grid.set(&Point::new(5, 5), Cell::Fill);

	view.draw();
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
	fn new () -> Self
	{
		Cell::Empty
	}
}


struct View
{
	grid: Grid<Cell, 50>,
	viewport: Area,
	camera: Arrow,
}

impl View
{
	fn new () -> View
	{
		let root   = Point::new(2, 2);
		let extent = Arrow::new(30, 20);
		let camera = Arrow::new(0, 0);

		View
		{
			grid: Grid::new(),
			viewport: Area { root, extent },
			camera,
		}
	}

	fn draw (&self)
	{
		self.clear();

		let (rows, cols) = self.viewport.extent.to_range();

		for row_n in rows
		{
			let root = self.viewport.root.clone();
			let root = root + Arrow::new(0, Base::try_from(row_n).unwrap());
			print!("{}", root.to_cursor());

			for col_n in cols.clone()
			{
				// println!("{};{}", row_n, col_n);
				// print!("{};{} ", row_n, col_n);

				let pin = Point::new(col_n, row_n) + self.camera;
				let cell = self.grid.get(&pin);

				print!("{}", match cell
				{
					Cell::Empty => '-',
					Cell::Fill  => 'X',
				});
			}
		}
	}

	fn clear (&self)
	{
		print!("{}", termion::clear::All);
	}
}
