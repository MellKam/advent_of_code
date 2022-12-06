mod crates_iter;
mod procedure;
mod supply_stacks;

use procedure::Procuedure;
use supply_stacks::SupplyStacks;

fn main() {
	let data = include_str!("../../input.txt");

	let (string_stacks, procedures_str) = data.split_once("\n\n").unwrap();

	let mut stacks = SupplyStacks::new(string_stacks);

	let procedures = procedures_str
		.lines()
		.map(|line| Procuedure::from(line))
		.collect::<Vec<Procuedure>>();

	for proc in procedures {
		stacks.apply_procedure(&proc, false);
	}

	let highest_vec = stacks.get_highest_elements();
	let highest_str = highest_vec.iter().collect::<String>();

	println!("{highest_str}");
}
