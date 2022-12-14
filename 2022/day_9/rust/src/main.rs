mod rope;

use rope::{parse_series_of_motions, Rope};

fn main() {
	let input = include_str!("../../input.txt");

	let mut rope = Rope::new(10, (0, 0));
	let motions = parse_series_of_motions(input);

	motions.iter().for_each(|motion| rope.apply_motion(motion));

	assert_eq!(rope.get_tails_visited_positions(), 2259);
}
