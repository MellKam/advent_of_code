use std::ptr::null_mut;

use super::{Directory, File};

#[derive(Debug)]
pub enum Command<'a> {
	CD(ChangeDirectory<'a>),
	LS(ListItems<'a>),
}

#[derive(Debug)]
pub enum ChangeDirectory<'a> {
	Root,
	Up,
	Down(&'a str),
}

#[derive(Debug)]
pub struct ListItems<'a> {
	pub dirs: Vec<&'a str>,
	pub files: Vec<File<'a>>,
}

#[derive(Debug)]
pub struct FileSystem<'a> {
	total_memory: u32,
	root_dir: Directory<'a>,
	pub cwd: *mut Directory<'a>,
}

impl<'a> FileSystem<'a> {
	pub fn new(total_memory: u32) -> Self {
		let mut fs = Self {
			total_memory,
			root_dir: Directory::new("", None),
			cwd: null_mut(),
		};

		fs.cwd = &mut fs.root_dir as *mut Directory<'a>;

		return fs;
	}

	pub fn chage_dir(&mut self, command: ChangeDirectory) {
		let next_dir = match command {
			ChangeDirectory::Down(dir_name) => unsafe {
				let dir_index = self
					.cwd
					.as_mut()
					.unwrap()
					.dirs
					.iter()
					.position(|d| d.name == dir_name)
					.unwrap();

				self.cwd.as_mut().unwrap().dirs.get_mut(dir_index).unwrap() as *mut Directory<'a>
			},
			ChangeDirectory::Root => &mut self.root_dir as *mut Directory<'a>,
			ChangeDirectory::Up => unsafe { self.cwd.as_mut().unwrap().parent_dir.unwrap() },
		};

		self.cwd = next_dir;
	}

	pub fn make_dir(&mut self, name: &'a str) -> *mut Directory<'a> {
		unsafe {
			if self.cwd.is_null() {
				return null_mut();
			}

			let dir = Directory::new(name, Some(self.cwd));

			return self.cwd.as_mut().unwrap().add_dir(dir);
		}
	}

	pub fn get_unused_space(&self) -> u32 {
		self.total_memory - self.get_system_size()
	}

	pub fn get_system_size(&self) -> u32 {
		self.root_dir.get_dir_size()
	}

	pub fn get_system_size_max(&self, max_size: u32) -> u32 {
		self.root_dir.get_dir_size_max(max_size)
	}

	pub fn get_smallest_folder_to_delete(&self, space_to_free: u32) -> Option<u32> {
		self.root_dir.get_smallest_folder_to_delete(space_to_free)
	}
}
