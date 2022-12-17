use std::fmt::Debug;

#[derive(Debug)]
enum Instruction {
	Addx(i32),
	Noop,
}

impl From<&str> for Instruction {
	fn from(string: &str) -> Self {
		match string.split_once(" ") {
			Some((_, n)) => Instruction::Addx(n.parse::<i32>().unwrap()),
			None => Instruction::Noop,
		}
	}
}

impl Instruction {
	pub fn cycles(&self) -> u32 {
		match self {
			Instruction::Noop => 1,
			Instruction::Addx(_) => 2,
		}
	}
}

struct Computer<'a> {
	x: i32,
	cycle: u32,
	on_cycle: Option<&'a mut dyn FnMut(u32, i32)>,
}

impl<'a> Computer<'a> {
	fn new(on_cycle: Option<&'a mut dyn FnMut(u32, i32)>) -> Self {
		Self {
			cycle: 0,
			x: 1,
			on_cycle,
		}
	}

	fn apply_instruction(&mut self, instruction: &Instruction) {
		for _ in 0..instruction.cycles() {
			self.cycle += 1;
			if self.on_cycle.is_some() {
				self.on_cycle.as_mut().unwrap()(self.cycle, self.x);
			}
		}

		match *instruction {
			Instruction::Addx(n) => self.x += n,
			Instruction::Noop => return,
		}
	}
}

fn main() {
	let data = include_str!("../../input.txt");

	let instructions = data
		.lines()
		.map(|line| Instruction::from(line))
		.collect::<Vec<Instruction>>();

	let connt_cycles: Vec<u32> = vec![20, 60, 100, 140, 180, 220];
	let mut sum: Vec<i32> = Vec::new();
	let mut on_cycle = |cycle: u32, x: i32| {
		if let Some(index) = connt_cycles.iter().position(|&c| c == cycle) {
			sum.push(x * connt_cycles[index] as i32);
		}
	};

	let mut computer = Computer::new(Some(&mut on_cycle));

	instructions.iter().for_each(|instruction| {
		computer.apply_instruction(instruction);
	});

	println!("{}", sum.iter().sum::<i32>());
}
