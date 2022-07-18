
use super::Point;
use super::Arrow;


#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub struct Area
{
	pub root: Point,
	pub extent: Arrow,
}
