
pub trait Cell: Copy
{
	fn empty () -> Self;

	fn draw (&self) -> char
	{
		'?'
	}
}
