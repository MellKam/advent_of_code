#[derive(Debug)]
struct Range {
	start: i32,
	end: i32,
}

impl From<&str> for Range {
	fn from(string: &str) -> Self {
		let (start, end) = match string.split_once("-") {
			Some((s, e)) => (s.parse::<i32>().unwrap(), e.parse::<i32>().unwrap()),
			_ => panic!(),
		};

		return Self { start, end };
	}
}

fn ranges_fully_contain(range1: &Range, range2: &Range) -> bool {
	if range1.start == range2.start || range1.end == range2.end {
		return true;
	}
	if range1.start < range2.start && range1.end > range2.end {
		return true;
	}
	if range2.end > range1.end {
		return true;
	}

	return false;
}

fn ranges_overlap(range1: &Range, range2: &Range) -> bool {
	if range1.start == range2.start
		|| range1.end == range2.end
		|| range1.start == range2.end
		|| range2.start == range1.end
	{
		return true;
	}
	if range1.start > range2.start && range2.end > range1.start {
		return true;
	}
	if range1.start < range2.start && range1.end > range2.start {
		return true;
	}

	return false;
}

fn main() {
	let data = include_str!("../../input.txt");

	let result: i32 = data
		.lines()
		.map(|line| {
			let (range1, range2) = match line.split_once(",") {
				Some((a, b)) => (Range::from(a), Range::from(b)),
				_ => panic!(),
			};

			return ranges_fully_contain(&range1, &range2) as i32;
		})
		.sum();

	let result2: i32 = data
		.lines()
		.map(|line| {
			let (range1, range2) = match line.split_once(",") {
				Some((a, b)) => (Range::from(a), Range::from(b)),
				_ => panic!(),
			};

			return ranges_overlap(&range1, &range2) as i32;
		})
		.sum();

	println!("{result}");
	println!("{result2}");
}
