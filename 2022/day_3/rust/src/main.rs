const CHARS_COUNT: usize = 52;
type AppearCharArray = [bool; CHARS_COUNT];

fn convert_ascii(ascii_char: u8) -> usize {
	if ascii_char > 96 && ascii_char < 123 {
		return (ascii_char - 96) as usize;
	}

	if ascii_char > 64 && ascii_char < 91 {
		return (ascii_char - 38) as usize;
	}

	panic!("Cannot parse '{ascii_char};");
}

fn get_appear_arr(string: &str) -> AppearCharArray {
	let mut store: AppearCharArray = [false; CHARS_COUNT];

	for byte in string.bytes() {
		let parsed_char = convert_ascii(byte);

		if !store[parsed_char - 1] {
			store[parsed_char - 1] = true;
		}
	}

	return store;
}

fn get_appear_sum(unique_chars_arrays: &Vec<AppearCharArray>) -> [u8; CHARS_COUNT] {
	let mut result: [u8; CHARS_COUNT] = [0; 52];

	for index in 0..CHARS_COUNT {
		let mut sum: u8 = 0;

		for array_index in 0..unique_chars_arrays.len() {
			sum += unique_chars_arrays[array_index][index] as u8;
		}

		result[index] = sum;
	}

	return result;
}

fn get_appear_chars_sum(array: &[u8; CHARS_COUNT], number: u8) -> i32 {
	let mut sum: i32 = 0;

	for (index, num) in array.into_iter().enumerate() {
		if *num == number {
			sum += index as i32 + 1;
		}
	}

	return sum;
}

fn main() {
	let data = include_str!("../../input.txt");

	let mut iter = data.lines();
	let mut sum: i32 = 0;

	loop {
		let lines = match (iter.next(), iter.next(), iter.next()) {
			(Some(a), Some(b), Some(c)) => [a, b, c],
			_ => break,
		};

		let appears = Vec::from(lines.map(|line| get_appear_arr(line)));

		let chars_sum = get_appear_chars_sum(&get_appear_sum(&appears), lines.len() as u8);
		sum += chars_sum;
	}

	let sum_2: i32 = data
		.lines()
		.map(|line| {
			let (left, right) = line.split_at(line.len() / 2);

			let appears = vec![get_appear_arr(left), get_appear_arr(right)];

			return get_appear_chars_sum(&get_appear_sum(&appears), 2);
		})
		.sum();

	println!("{sum}");
	println!("{sum_2}");
}
