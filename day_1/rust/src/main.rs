mod quick_sort;

use quick_sort::quick_sort;
use std::fs;

// Part one
fn find_largest_calorie_size(calorie_list: &String) -> i32 {
	let mut max: i32 = 0;
	let mut temp_sum: i32 = 0;
	for line in calorie_list.lines() {
		if line == "" {
			if temp_sum > max {
				max = temp_sum
			};
			temp_sum = 0;
			continue;
		}
		temp_sum += line.parse::<i32>().unwrap();
	}

	return max;
}

// Part two
fn find_three_largest_calorie_size(calorie_list: &String) -> i32 {
	let mut total_calories = Vec::<i32>::new();
	let mut temp_sum: i32 = 0;

	for line in calorie_list.lines() {
		if line == "" {
			total_calories.push(temp_sum);
			temp_sum = 0;
			continue;
		}

		temp_sum += line.parse::<i32>().unwrap();
	}

	quick_sort(&mut total_calories);

	let length = total_calories.len();

	return total_calories[length - 3] + total_calories[length - 2] + total_calories[length - 1];
}

fn main() {
	let data = fs::read_to_string("../input.txt").expect("Error while trying to read input file");

	let max_total = find_largest_calorie_size(&data);
	let three_max_total = find_three_largest_calorie_size(&data);

	println!("max total: {max_total}");
	println!("three max total: {three_max_total}");
}
