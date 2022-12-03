mod quick_sort;

use quick_sort::quick_sort;

fn get_total_calories(calorie_list: &str) -> Vec<i32> {
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
	return total_calories;
}

fn main() {
	let data = include_str!("../../input.txt");

	let total_calories = get_total_calories(data);
	let length = total_calories.len();

	let max = total_calories[length - 1];
	let three_max = [
		total_calories[length - 3],
		total_calories[length - 2],
		total_calories[length - 1],
	];

	println!("max total: {max}");
	println!("three max total: {three_max:#?}");
}
