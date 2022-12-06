use crate::{crates_iter::CratesLineIterator, procedure::Procuedure};

#[derive(Debug)]
pub struct SupplyStacks {
	stacks: Vec<Vec<char>>,
}

impl SupplyStacks {
	pub fn new(string_stacks: &str) -> Self {
		let mut stacks_of_cartes = Self { stacks: Vec::new() };
		stacks_of_cartes.init_stacks(string_stacks);

		return stacks_of_cartes;
	}

	fn init_stacks(&mut self, string_stacks: &str) {
		let mut stacks_iter = string_stacks.lines().rev();

		for char in stacks_iter.next().unwrap().chars() {
			if char == ' ' {
				continue;
			}
			self.stacks.push(Vec::new());
		}

		for line in stacks_iter {
			for (index, char) in CratesLineIterator::new(line) {
				self.stacks[index - 1].push(char);
			}
		}
	}

	pub fn apply_procedure(&mut self, procedure: &Procuedure, save_order: bool) {
		if procedure.move_from == procedure.move_to {
			return;
		}

		let from_length = self.stacks[procedure.move_from - 1].len();

		let moved_elements: Vec<char>;

		if save_order {
			moved_elements = self.stacks[procedure.move_from - 1]
				.drain((from_length - procedure.crates_count as usize)..)
				.rev()
				.collect::<Vec<char>>();
		} else {
			moved_elements = self.stacks[procedure.move_from - 1]
				.drain((from_length - procedure.crates_count as usize)..)
				.collect::<Vec<char>>();
		}

		let to_index = procedure.move_to - 1;

		for char in moved_elements {
			self.stacks[to_index].push(char);
		}
	}

	pub fn get_highest_elements(&self) -> Vec<char> {
		let mut result: Vec<char> = Vec::new();

		for stack in &self.stacks {
			result.push(stack[stack.len() - 1])
		}

		return result;
	}
}
