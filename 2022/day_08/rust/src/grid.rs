use std::ops::Range;

use crate::{grid_peak_ranges::GridPeakRanges, scenic_score::ScenicScore};

#[derive(Debug)]
pub struct Grid {
	pub rows: usize,
	pub cols: usize,
	pub grid: Vec<Vec<u8>>,
}

impl From<&str> for Grid {
	fn from(string_grid: &str) -> Self {
		let mut grid: Vec<Vec<u8>> = Vec::new();

		for line in string_grid.lines() {
			let grid_line = line
				.chars()
				.map(|char| char.to_digit(10).unwrap().try_into().unwrap())
				.collect::<Vec<u8>>();

			grid.push(grid_line);
		}

		let rows = grid.len();
		let cols = grid[0].len();

		Self { grid, rows, cols }
	}
}

impl Grid {
	pub fn get_visible_items_map(&self, peak_ranges: &GridPeakRanges) -> Vec<Vec<bool>> {
		let mut visibility_map: Vec<Vec<bool>> = Vec::with_capacity(self.rows);

		for row_index in 0..self.rows {
			let mut visibility_row: Vec<bool> = vec![false; self.cols];

			Grid::get_visible_from_line(
				|i| self.grid[row_index][i],
				|i| visibility_row[i] = true,
				peak_ranges.x.get(row_index).unwrap(),
				self.cols,
			);

			visibility_map.push(visibility_row);
		}

		for col_index in 0..self.cols {
			Grid::get_visible_from_line(
				|i| self.grid[i][col_index],
				|i| visibility_map[i][col_index] = true,
				peak_ranges.y.get(col_index).unwrap(),
				self.rows,
			);
		}

		return visibility_map;
	}

	fn get_visible_from_line<LineGetter: Fn(usize) -> u8, VisibilitySetter: FnMut(usize)>(
		line_getter: LineGetter,
		mut visible_setter: VisibilitySetter,
		peak_range: &Range<usize>,
		line_width: usize,
	) {
		let mut max_num: u8;

		let mut left_iter = (0..peak_range.start + 1).step_by(1);
		let mut right_iter = (peak_range.end..line_width).step_by(1).rev();

		let first_left_num = left_iter.next().unwrap();
		visible_setter(first_left_num);
		max_num = line_getter(first_left_num);

		for index in left_iter {
			let num = line_getter(index);

			if num > max_num {
				visible_setter(index);
				max_num = num;
			}
		}

		let first_right_num = right_iter.next().unwrap();
		visible_setter(first_right_num);
		max_num = line_getter(first_right_num);

		for index in right_iter {
			let num = line_getter(index);

			if num > max_num {
				visible_setter(index);
				max_num = num;
			}
		}
	}

	pub fn get_scenic_scores(&self) -> Vec<u32> {
		let mut scenic_scores: Vec<u32> = Vec::with_capacity(self.rows * self.cols);

		for row_index in 0..self.rows {
			for col_index in 0..self.cols {
				let height = self.grid[row_index][col_index];

				let right_score = Grid::get_direction_scenic_score(
					height,
					(col_index + 1..self.cols).into_iter(),
					|col_i| self.grid[row_index][col_i],
				);

				let left_score =
					Grid::get_direction_scenic_score(height, (0..col_index).into_iter().rev(), |col_i| {
						self.grid[row_index][col_i]
					});

				let top_score =
					Grid::get_direction_scenic_score(height, (0..row_index).into_iter().rev(), |row_i| {
						self.grid[row_i][col_index]
					});

				let bottom_score = Grid::get_direction_scenic_score(
					height,
					(row_index + 1..self.rows).into_iter(),
					|row_i| self.grid[row_i][col_index],
				);

				let scenic_score =
					ScenicScore::new(right_score, left_score, top_score, bottom_score).get_total_score();

				scenic_scores.push(scenic_score);
			}
		}

		return scenic_scores;
	}

	fn get_direction_scenic_score<ItemGetter: Fn(usize) -> u8>(
		height: u8,
		index_iter: impl Iterator<Item = usize>,
		item_getter: ItemGetter,
	) -> u32 {
		let mut scenic_score: u32 = 0;

		for i in index_iter {
			let h = item_getter(i);

			if h >= height {
				return scenic_score + 1;
			}

			scenic_score += 1;
		}

		if scenic_score == 0 {
			return 1;
		}

		return scenic_score;
	}
}
