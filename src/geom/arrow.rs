
use super::Pair;
use super::Offset;

pub type Arrow = Pair<Offset>;


//
// Arrow + Arrow = Arrow
//
impl std::ops::Add<Arrow> for Arrow
{
	type Output = Self;

	fn add (self, arrow: Arrow) -> Self
	{
		Arrow::new(self.x.0 + arrow.x.0, self.y.0 + arrow.y.0)
	}
}

//
// Arrow - Arrow = Arrow
//
impl std::ops::Sub<Arrow> for Arrow
{
	type Output = Self;

	fn sub (self, arrow: Arrow) -> Self
	{
		Arrow::new(self.x.0 - arrow.x.0, self.y.0 - arrow.y.0)
	}
}

//
// -Arrow = Arrow
//
impl std::ops::Neg for Arrow
{
	type Output = Self;

	fn neg (self) -> Self
	{
		Arrow::new(-self.x.0, -self.y.0)
	}
}
