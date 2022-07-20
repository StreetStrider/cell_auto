
pub trait Cell
{
	fn empty () -> Self;

	fn draw (&self) -> char
	{
		'?'
	}
}
