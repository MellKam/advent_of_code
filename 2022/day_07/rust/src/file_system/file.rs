#[derive(Debug)]
pub struct File<'a> {
	pub name: &'a str,
	pub size: u32,
}

impl<'a> File<'a> {
	pub fn new(name: &'a str, size: u32) -> Self {
		Self { name, size }
	}
}
