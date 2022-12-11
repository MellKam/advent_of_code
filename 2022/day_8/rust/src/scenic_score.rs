#[derive(Debug)]
pub struct ScenicScore {
	pub top: u32,
	pub bottom: u32,
	pub left: u32,
	pub right: u32,
}

impl ScenicScore {
	pub fn new(right: u32, left: u32, top: u32, bottom: u32) -> Self {
		Self {
			bottom,
			left,
			right,
			top,
		}
	}

	pub fn get_total_score(&self) -> u32 {
		self.bottom * self.top * self.left * self.right
	}
}
