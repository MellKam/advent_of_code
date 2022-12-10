use crate::Grid;
use std::ops::Range;

#[derive(Debug)]
pub struct GridPeakRanges {
	pub x: Vec<Range<usize>>,
	pub y: Vec<Range<usize>>,
}

impl From<&Grid> for GridPeakRanges {
	fn from(grid: &Grid) -> Self {
		let mut peak_ranges_x: Vec<Range<usize>> = Vec::with_capacity(grid.rows);

		for row_index in 0..grid.rows {
			let mut max_num: u8 = 0;
			let mut peak_range: Range<usize> = Range { start: 0, end: 0 };

			for col_index in 0..grid.cols {
				let num = grid.grid[row_index][col_index];

				if num > max_num {
					peak_range.start = col_index;
					peak_range.end = col_index;
					max_num = num;
				}

				if num == max_num {
					peak_range.end = col_index;
				}
			}

			peak_ranges_x.push(peak_range);
		}

		let mut peak_ranges_y: Vec<Range<usize>> = Vec::with_capacity(grid.cols);

		for col_index in 0..grid.cols {
			let mut max_num: u8 = 0;
			let mut peak_range: Range<usize> = Range { start: 0, end: 0 };

			for row_index in 0..grid.rows {
				let num = grid.grid[row_index][col_index];

				if num > max_num {
					peak_range.start = row_index;
					peak_range.end = row_index;
					max_num = num;
				}

				if num == max_num {
					peak_range.end = row_index;
				}
			}

			peak_ranges_y.push(peak_range);
		}

		Self {
			x: peak_ranges_x,
			y: peak_ranges_y,
		}
	}
}
