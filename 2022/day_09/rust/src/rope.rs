use std::{
	cmp::{max, min},
	collections::HashSet,
	ops::Range,
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
	knots: Vec<(i32, i32)>,
	tails_visited_positions: HashSet<(i32, i32)>,
}

pub fn get_vectors_delta(v1: &(i32, i32), v2: &(i32, i32)) -> (i32, i32) {
	(
		max(v1.0, v2.0) - min(v1.0, v2.0),
		max(v1.1, v2.1) - min(v1.1, v2.1),
	)
}

impl Rope {
	pub fn new(knots_count: u8, start_point: (i32, i32)) -> Self {
		let mut tails_visited_positions = HashSet::new();
		tails_visited_positions.insert(start_point);

		let mut knots: Vec<(i32, i32)> = Vec::new();

		for _ in 0..knots_count {
			knots.push(start_point);
		}

		Self {
			knots,
			tails_visited_positions,
		}
	}

	fn is_ropes_touch(delta: &(i32, i32)) -> bool {
		return !(delta.0 > 1 || delta.1 > 1);
	}

	pub fn move_knot(&mut self, delta: &(i32, i32), top_knot_index: usize) {
		if delta.0 >= 1 {
			if self.knots[top_knot_index].0 > self.knots[top_knot_index + 1].0 {
				self.knots[top_knot_index + 1].0 += 1;
			} else {
				self.knots[top_knot_index + 1].0 -= 1;
			}
		}

		if delta.1 >= 1 {
			if self.knots[top_knot_index].1 > self.knots[top_knot_index + 1].1 {
				self.knots[top_knot_index + 1].1 += 1;
			} else {
				self.knots[top_knot_index + 1].1 -= 1;
			}
		}
	}

	pub fn apply_motion(&mut self, motion: &Motion) {
		match *motion {
			Motion::Down => self.knots[0].1 -= 1,
			Motion::Left => self.knots[0].0 -= 1,
			Motion::Right => self.knots[0].0 += 1,
			Motion::Up => self.knots[0].1 += 1,
		}

		let mut iter = (0..self.knots.len() - 1).into_iter();

		loop {
			let i = match iter.next() {
				Some(i) => i,
				None => break,
			};

			let delta = get_vectors_delta(&self.knots[i], &self.knots[i + 1]);
			
			if Rope::is_ropes_touch(&delta) {
				break;
			}

			self.move_knot(&delta, i);
		}

		self
			.tails_visited_positions
			.insert(*self.knots.last().unwrap());
	}

	pub fn get_tails_visited_positions(&self) -> usize {
		self.tails_visited_positions.len()
	}

	pub fn print_rope(&self) {
		let y_range = -16..16;
		let x_range = -16..16;

		for y in y_range.rev() {
			for x in x_range.clone() {
				let i = self.knots.iter().position(|&p| p == (x + 1, y + 1));
				match i {
					Some(i) => print!("{i}"),
					None => print!("."),
				}
			}
			print!("\n");
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn demo() {
		let input = "\
			R 5\n\
			U 8\n\
			L 8\n\
			D 3\n\
			R 17\n\
			D 10\n\
			L 25\n\
			";

		let mut rope = Rope::new(10, (0, 0));
		let motions = parse_series_of_motions(input);

		motions.iter().for_each(|motion| rope.apply_motion(motion));

		rope.print_rope();

		println!("{:?}", rope.knots);

		assert_eq!(rope.tails_visited_positions.len(), 36);
	}
}
