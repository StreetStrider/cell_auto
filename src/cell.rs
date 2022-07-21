
pub trait Cell: Eq + Copy
{
	fn empty () -> Self;

	fn draw (&self) -> char
	{
		'?'
	}
}
