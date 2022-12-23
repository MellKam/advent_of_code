#[derive(Debug)]
pub struct Procuedure {
	pub move_from: usize,
	pub move_to: usize,
	pub crates_count: u32,
}

impl From<&str> for Procuedure {
	fn from(string: &str) -> Self {
		let items = string.split(" ").collect::<Vec<&str>>();

		return Procuedure {
			crates_count: items[1].parse::<u32>().unwrap(),
			move_from: items[3].parse::<usize>().unwrap(),
			move_to: items[5].parse::<usize>().unwrap(),
		};
	}
}
