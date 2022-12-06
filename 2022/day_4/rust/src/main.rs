use std::{fmt::Debug, ops::Range, str::FromStr};

fn parse_to_range<T: Sized + FromStr>(string: &str) -> Range<T>
where
	T::Err: Debug,
{
	let (start, end) = match string.split_once("-") {
		Some((s, e)) => (s.parse::<T>().unwrap(), e.parse::<T>().unwrap()),
		_ => panic!("Cannot parse string to range"),
	};

	return Range { start, end };
}

#[derive(Debug)]
struct RangePair<T: Sized> {
	r1: Range<T>,
	r2: Range<T>,
}

impl<T: Sized + FromStr> From<&str> for RangePair<T>
where
	T::Err: Debug,
{
	fn from(string: &str) -> Self {
		let (r1, r2) = match string.split_once(",") {
			Some((a, b)) => (parse_to_range::<T>(a), parse_to_range::<T>(b)),
			_ => panic!("Cannot split string"),
		};

		Self { r1, r2 }
	}
}

impl<T: PartialOrd> RangePair<T> {
	fn contain(&self) -> bool {
		if self.r1.start <= self.r2.start && self.r1.end >= self.r2.end {
			// then r1 is outer range and r2 is inner
			return true;
		}
		if self.r2.start <= self.r1.start && self.r2.end >= self.r1.end {
			// then r2 is outer range and r1 in inner
			return true;
		}

		return false;
	}

	fn overlap(&self) -> bool {
		if self.r1.start > self.r2.start && self.r2.end > self.r1.start {
			return true;
		}
		if self.r1.start < self.r2.start && self.r1.end > self.r2.start {
			return true;
		}
		if self.r1.start == self.r2.start
			|| self.r1.end == self.r2.end
			|| self.r1.start == self.r2.end
			|| self.r2.start == self.r1.end
		{
			return true;
		}

		return false;
	}
}

fn main() {
	let data = include_str!("../../input.txt");

	let contains: u32 = data
		.lines()
		.map(|line| RangePair::<u32>::from(line).contain() as u32)
		.sum();

	let overlaps: u32 = data
		.lines()
		.map(|line| RangePair::<u32>::from(line).overlap() as u32)
		.sum();

	println!("{contains}");
	println!("{overlaps}");
}
