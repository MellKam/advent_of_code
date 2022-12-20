use std::fmt::{Debug, Display};

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

#[derive(Debug, Clone, Copy)]
enum Pixel {
	Black,
	White,
}

impl Pixel {
	pub fn symbol(&self) -> char {
		match self {
			Pixel::Black => '.',
			Pixel::White => '#',
		}
	}
}

#[derive(Debug)]
struct Screen {
	row_width: usize,
	display: Vec<Pixel>,
}

impl Screen {
	pub fn new(row_width: usize) -> Self {
		Self {
			row_width,
			display: Vec::new(),
		}
	}

	pub fn fill_pixel(&mut self, cycle: u32, x: i32) {
		let c = (cycle as i32 - 1) % 40;

		if ((x - 1)..=(x + 1)).contains(&c) {
			self.display.push(Pixel::White);
		} else {
			self.display.push(Pixel::Black);
		}
	}
}

impl Display for Screen {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			self
				.display
				.chunks(self.row_width)
				.map(|chunk| chunk.iter().map(|x| x.symbol()).collect::<String>())
				.collect::<Vec<String>>()
				.join("\n")
		)
	}
}

fn main() {
	let data = include_str!("../../input.txt");

	let instructions = data
		.lines()
		.map(|line| Instruction::from(line))
		.collect::<Vec<Instruction>>();

	// let connt_cycles: Vec<u32> = vec![20, 60, 100, 140, 180, 220];
	// let mut sum: Vec<i32> = Vec::new();
	// let mut on_cycle = |cycle: u32, x: i32| {
	// 	if let Some(index) = connt_cycles.iter().position(|&c| c == cycle) {
	// 		sum.push(x * connt_cycles[index] as i32);
	// 	}
	// };

	let mut screen = Screen::new(40);

	let mut on_cycle = |cycle: u32, x: i32| {
		screen.fill_pixel(cycle, x);
	};

	let mut computer = Computer::new(Some(&mut on_cycle));

	instructions.iter().for_each(|instruction| {
		computer.apply_instruction(instruction);
	});

	println!("{}", screen);
}
