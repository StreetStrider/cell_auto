
use super::Arrow;
use super::Point;
use super::Cell;
use super::Grid;

pub type Moore = [Arrow; 8];

pub fn fill <C: Cell> (moore: &Moore, grid: &impl Grid<Cell = C>, point: &Point) -> usize
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

pub fn new () -> Moore
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
