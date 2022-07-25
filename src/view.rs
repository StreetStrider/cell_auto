
use termion;
use termion::cursor::Goto;

use tempus_fugit::Measurement;

use super::Cell;

use super::geom::Offset;

use super::geom::Point;
use super::geom::Arrow;

use super::geom::grid::Grid;


const termion_goto_arrow: Arrow = Arrow { x: Offset(1), y: Offset(1) };


pub struct View
{
	pub gen: u32,

	pub m_cycle: Measurement,
	pub m_draw: Measurement,

	root: Arrow,
	pub camera: Arrow,
}

impl View
{
	pub fn new () -> View
	{
		let gen = 0;

		let m_cycle = Measurement::zero();
		let m_draw = Measurement::zero();

		let root = Arrow::new(0, 1);
		let camera = Arrow::new(0, 0);
		// let camera = Arrow::new(4, 3);

		View
		{
			gen,
			m_cycle,
			m_draw,
			root,
			camera,
		}
	}

	pub fn tick (&mut self)
	{
		self.gen = (self.gen + 1);
	}

	pub fn draw <C: Cell> (&self, grid: &impl Grid<Cell = C>)
	{
		self.clear();

		print!("gen {}; cycle ({}); draw ({})", self.gen, self.m_cycle, self.m_draw);

		let view_size = terminal_size() - self.root - Arrow::new(0, 1);
		let (rows, cols) = view_size.to_range();

		let term_root = (Point::zero() + self.root + termion_goto_arrow);
		let grid_root = (Point::zero() + self.camera);

		for row_n in rows
		{
			let p_term = (term_root + Arrow::new(0, row_n));
			print!("{}", Goto::from(p_term));

			for col_n in cols.clone()
			{
				let p_grid = (grid_root + Arrow::new(col_n, row_n));
				let cell = grid.get(&p_grid);

				if let Some(cell) = cell
				{
					print!("{}", cell.draw());
				}
			}
			print!("\n");
		}
	}

	fn clear (&self)
	{
		print!("{}", termion::clear::All);
		print!("{}", Goto(1, 1));
	}
}


fn terminal_size () -> Arrow
{
	Arrow::from(termion::terminal_size().unwrap())
}
