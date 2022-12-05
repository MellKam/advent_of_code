use std::str::Chars;

pub struct CratesLineIterator<'a> {
	iter: Chars<'a>,
	stack_index: u32,
}

impl<'a> CratesLineIterator<'a> {
	fn skip_by(&mut self, n: usize) -> Result<(), usize> {
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
		if let Err(_) = self.skip_by(1) {
			return None;
		}
		let optional_char = self.iter.next();

		if let Err(size) = self.skip_by(2) {
			if size != 1 {
				return None;
			}
		}

		return match optional_char {
			Some(' ') => {
				self.stack_index += 1;
				return self.next();
			}
			Some(c) => {
				self.stack_index += 1;
				Some((self.stack_index as usize, c))
			}
			None => return None,
		};
	}
}
