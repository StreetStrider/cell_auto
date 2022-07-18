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


type TermScalar = u16;


fn main ()
{
	let view = View::new();

	// let _origin = Point::new(2, 2);
	// view.grid[3][4] = Cell::Fill;

	view.draw();
}

#[derive(Clone)]
#[derive(Copy)]
enum Cell
{
	Empty,
	Fill,
}

const size: usize = 30;

type Row   = [ Cell; size ];
type Table = [  Row; size ];

struct Grid
{
	table: Table,
}

impl Grid
{
	fn new () -> Grid
	{
		let mut grid = Grid { table: [ [ Cell::Empty; size ]; size ] };
		grid.table[3][4] = Cell::Fill;

		return grid
	}
}

struct View
{
	grid: Grid,
	area: Area,
}

impl View
{
	fn new () -> View
	{
		let root   = Point::new(2, 2);
		let extent = Arrow::new(30, 20);

		View
		{
			grid: Grid::new(),
			area: Area { root, extent },
		}
	}

	fn draw (&self)
	{
		self.clear();

		for (row_n, row) in self.grid.table.iter().enumerate()
		{
			let root = self.area.root.clone();
			let root = root + Arrow::new(0, Base::try_from(row_n).unwrap());
			print!("{}", root.to_cursor());

			for cell in row
			{
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
