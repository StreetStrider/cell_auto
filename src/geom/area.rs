
use std::ops::Range;

use super::Base;
use super::Point;
use super::Arrow;


#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(Debug)]
pub struct Area
{
	pub root: Point,
	pub extent: Arrow,
}

impl Area
{
	pub fn to_range (&self) -> (Range<Base>, Range<Base>)
	{
		let rows = self.root.y.0 .. (self.root.y.0 + self.extent.y.0);
		let cols = self.root.x.0 .. (self.root.x.0 + self.extent.x.0);

		(rows, cols)
	}
}
