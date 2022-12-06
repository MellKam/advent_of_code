fn find_start_of_packet(data: &str, marker_length: usize) -> usize {
	if data.len() < marker_length {
		return 0;
	}

	let bytes_data = data.as_bytes();
	let mut curr_index = 0;
	let mut unique_count = 0;

	return 'outer: loop {
		let active_byte = match bytes_data.get(curr_index) {
			Some(b) => b,
			None => break 'outer 0,
		};

		let max_marker_index = curr_index + (marker_length - 1) - unique_count;

		'inner: loop {
			curr_index += 1;

			if *active_byte == *bytes_data.get(curr_index).unwrap() {
				unique_count = 0;
				curr_index = max_marker_index - (marker_length - 2);
				break 'inner;
			}

			if curr_index == max_marker_index {
				unique_count += 1;

				if unique_count == marker_length - 1 {
					break 'outer curr_index + 1;
				}

				curr_index -= (marker_length - 1) - unique_count;
				break 'inner;
			}
		}
	};
}

fn main() {
	let data = include_str!("../../input.txt");

	let result = find_start_of_packet(data, 14);
	println!("{result}");
}
