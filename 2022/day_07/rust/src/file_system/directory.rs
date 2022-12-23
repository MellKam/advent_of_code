use super::File;

#[derive(Debug)]
pub struct Directory<'a> {
	pub name: &'a str,
	pub parent_dir: Option<*mut Directory<'a>>,
	pub files: Vec<File<'a>>,
	pub dirs: Vec<Directory<'a>>,
}

impl<'a> Directory<'a> {
	pub fn new(name: &'a str, parent_dir: Option<*mut Directory<'a>>) -> Self {
		Self {
			name,
			parent_dir,
			dirs: Vec::new(),
			files: Vec::new(),
		}
	}

	pub fn add_file(&mut self, file: File<'a>) {
		self.files.push(file);
	}

	pub fn add_dir(&mut self, dir: Directory<'a>) -> *mut Directory<'a> {
		let dir_index = self.dirs.len();
		self.dirs.push(dir);

		return self.dirs.get_mut(dir_index).unwrap() as *mut Directory;
	}

	pub fn get_dir_size(&self) -> u32 {
		let mut size: u32 = 0;

		if self.files.len() != 0 {
			size += self.files.iter().map(|f| f.size).sum::<u32>()
		}

		if self.dirs.len() != 0 {
			size += self.dirs.iter().map(|d| d.get_dir_size()).sum::<u32>();
		}

		return size;
	}

	pub fn get_dir_size_max(&self, max_size: u32) -> u32 {
		let mut size = 0;

		let dir_size = self.get_dir_size();
		if dir_size <= max_size {
			size += dir_size;
		}

		if self.dirs.len() != 0 {
			size += self
				.dirs
				.iter()
				.map(|d| d.get_dir_size_max(max_size))
				.sum::<u32>();
		}

		return size;
	}

	pub fn get_smallest_folder_to_delete(&self, space_to_free: u32) -> Option<u32> {
		let mut result = None;

		let curr_dir_size = self.get_dir_size();
		if curr_dir_size >= space_to_free {
			result = Some(curr_dir_size);
		}

		if self.dirs.len() > 0 && result != None {
			let min_child = self
				.dirs
				.iter()
				.map(|d| d.get_smallest_folder_to_delete(space_to_free))
				.filter(|size| size.is_some())
				.map(|size| size.unwrap())
				.min();

			if min_child != None && min_child.unwrap() < result.unwrap() {
				return min_child;
			}
		}

		return result;
	}
}
