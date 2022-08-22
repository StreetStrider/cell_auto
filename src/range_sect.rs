
use std::ops::Range;

pub type Sect = Vec<Range<usize>>;

pub fn sect (size: usize, buckets: usize) -> Sect
{
	(0..buckets).into_iter().map(|num|
	{
		let start = ((size * (0 + num)) / buckets);
		let end   = ((size * (1 + num)) / buckets);

		start..end
	})
	.collect()
}
