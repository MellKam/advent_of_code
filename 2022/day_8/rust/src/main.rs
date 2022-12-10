mod grid;
mod grid_peak_ranges;

use grid::Grid;
use grid_peak_ranges::GridPeakRanges;

fn main() {
	let data = include_str!("../../input.txt");

	let grid = Grid::from(data);
	let grid_peak_ranges = GridPeakRanges::from(&grid);

	let visibility_map = grid.get_visible_items_map(&grid_peak_ranges);

	let sum = visibility_map
		.iter()
		.map(|row| row.iter().map(|&v| v as u32).sum::<u32>())
		.sum::<u32>();

	println!("{sum}");
}
