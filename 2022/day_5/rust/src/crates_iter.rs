use std::str::Chars;

pub struct CratesLineIterator<'a> {
	iter: Chars<'a>,
	stack_index: u32,
}

impl<'a> CratesLineIterator<'a> {
	fn advance(&mut self, n: usize) -> Result<(), usize> {
		for i in 0..n {
			self.iter.next().ok_or(i)?;
		}
		Ok(())
	}

	pub fn new(string: &'a str) -> Self {
		Self {
			iter: string.chars(),
			stack_index: 0,
		}
	}
}

impl Iterator for CratesLineIterator<'_> {
	type Item = (usize, char);

	fn next(&mut self) -> Option<Self::Item> {
		if let Err(_) = self.advance(1) {
			return None;
		}
		let current_char = self.iter.next()?;

		if let Err(size) = self.advance(2) {
			if size != 1 {
				return None;
			}
		}

		self.stack_index += 1;

		if current_char == ' ' {
			return self.next();
		}

		return Some((self.stack_index as usize, current_char));
	}
}
