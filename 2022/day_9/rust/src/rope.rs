use std::{
	cmp::{max, min},
	collections::HashSet,
};

#[derive(Debug, Clone, Copy)]
pub enum Motion {
	Right,
	Left,
	Up,
	Down,
}

impl From<&str> for Motion {
	fn from(string: &str) -> Self {
		match string {
			"R" => Motion::Right,
			"U" => Motion::Up,
			"D" => Motion::Down,
			"L" => Motion::Left,
			_ => panic!(),
		}
	}
}

pub fn parse_series_of_motions(string: &str) -> Vec<Motion> {
	let mut motions: Vec<Motion> = Vec::new();

	for line in string.lines() {
		let (motion, count) = match line.split_once(" ") {
			Some((m, c)) => (Motion::from(m), c.parse::<u8>().unwrap()),
			_ => panic!(),
		};

		for _ in 0..count {
			motions.push(motion);
		}
	}

	return motions;
}

#[derive(Debug)]
pub struct Rope {
	head: (i32, i32),
	tail: (i32, i32),
	tails_visited_positions: HashSet<(i32, i32)>,
}

pub fn get_vectors_delta(v1: (i32, i32), v2: (i32, i32)) -> (i32, i32) {
	(
		max(v1.0, v2.0) - min(v1.0, v2.0),
		max(v1.1, v2.1) - min(v1.1, v2.1),
	)
}

impl Rope {
	pub fn new() -> Self {
		let mut tails_visited_positions = HashSet::new();
		tails_visited_positions.insert((0, 0));

		Self {
			head: (0, 0),
			tail: (0, 0),
			tails_visited_positions,
		}
	}

	fn get_rope_state(&self) -> (bool, (i32, i32)) {
		let delta = get_vectors_delta(self.head, self.tail);
		return (!(delta.0 > 1 || delta.1 > 1), delta);
	}

	pub fn move_tail_to_head(&mut self, delta: (i32, i32)) {
		if delta.0 >= 1 {
			if self.head.0 > self.tail.0 {
				self.tail.0 += 1;
			} else {
				self.tail.0 -= 1;
			}
		}

		if delta.1 >= 1 {
			if self.head.1 > self.tail.1 {
				self.tail.1 += 1;
			} else {
				self.tail.1 -= 1;
			}
		}
	}

	pub fn apply_motion(&mut self, motion: &Motion) {
		match *motion {
			Motion::Down => self.head.1 -= 1,
			Motion::Left => self.head.0 -= 1,
			Motion::Right => self.head.0 += 1,
			Motion::Up => self.head.1 += 1,
		}

		let (is_touch, delta) = self.get_rope_state();

		if !is_touch {
			self.move_tail_to_head(delta);
			self.tails_visited_positions.insert(self.tail);
		}
	}

	pub fn get_tails_visited_positions(&self) -> usize {
		self.tails_visited_positions.len()
	}
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn demo() {
		let input = "\
			R 4\n\
			U 4\n\
			L 3\n\
			D 1\n\
			R 4\n\
			D 1\n\
			L 5\n\
			R 2";

		let mut rope = Rope::new();
		let motions = parse_series_of_motions(input);

		motions.iter().for_each(|motion| rope.apply_motion(motion));

		assert_eq!(rope.tails_visited_positions.len(), 13);
	}
}
