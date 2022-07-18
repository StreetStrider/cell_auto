
use super::Base;
use super::Scalar;


#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(Debug)]
pub struct Offset(pub Base);


impl Scalar for Offset
{
	fn new (value: Base) -> Self
	{
		Self(value)
	}

	fn to_usize (&self) -> usize
	{
		self.0 as usize
	}
}


//
// Offset + Offset = Offset
//
impl std::ops::Add for Offset
{
	type Output = Offset;

	fn add (self, R: Offset) -> Self
	{
		Offset(self.0 + R.0)
	}
}

//
// Offset - Offset = Offset
//
impl std::ops::Sub for Offset
{
	type Output = Offset;

	fn sub (self, R: Offset) -> Self
	{
		Offset(self.0 - R.0)
	}
}

//
// -Offset = Offset
//
impl std::ops::Neg for Offset
{
	type Output = Offset;

	fn neg (self) -> Self
	{
		Offset(-self.0)
	}
}

impl Offset
{
	pub fn new (value: Base) -> Offset
	{
		Offset(value)
	}
}
