
use termion;

use super::Cell;

use super::geom::Base;
use super::geom::Offset;

use super::geom::Point;
use super::geom::Arrow;

use super::geom::grid::GridRead;


const termion_goto_arrow: Arrow = Arrow { x: Offset(1), y: Offset(1) };


pub struct View
{
	pub gen: u32,

	root: Arrow,
	pub camera: Arrow,
}

impl View
{
	pub fn new () -> View
	{
		let gen = 0;
		let root = Arrow::new(0, 1);
		let camera = Arrow::new(0, 0);
		// let camera = Arrow::new(4, 3);

		View
		{
			gen,
			root,
			camera,
		}
	}

	pub fn tick (&mut self)
	{
		self.gen = (self.gen + 1);
	}

	pub fn draw <C: Cell> (&self, grid: &dyn GridRead<Item = C>)
	{
		self.clear();

		print!("gen {}", self.gen);

		let view_size = terminal_size() - self.root - Arrow::new(0, 5);
		let (rows, cols) = view_size.to_range();
		// print!(" ({:?};{:?})", cols, rows);

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

				let cell = grid.get(&p_grid);

				if let Some(cell) = cell
				{
					print!("{}", cell.draw());
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
