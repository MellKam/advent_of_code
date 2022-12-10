use crate::file_system::{ChangeDirectory, Command, File, FileSystem, ListItems};

pub struct FileSystemParser<'a> {
	pub fs: FileSystem<'a>,
}

impl<'a> FileSystemParser<'a> {
	pub fn new(fs: FileSystem<'a>) -> Self {
		Self { fs }
	}

	pub fn parse_commands_string(&self, string: &'a str) -> Vec<Command<'a>> {
		let commands: Vec<Command> = string
			.split("$ ")
			.skip(1)
			.map(|command_str| {
				let command_vector = command_str.trim().split("\n").collect::<Vec<&str>>();

				if command_vector[0] == "ls" {
					let ls_output = command_vector.iter().skip(1);
					let mut ls = ListItems {
						dirs: Vec::new(),
						files: Vec::new(),
					};

					for &line in ls_output {
						let mut iter = line.split(" ");

						match iter.next() {
							Some("dir") => {
								ls.dirs.push(iter.next().unwrap());
							}
							Some(num) => {
								ls.files
									.push(File::new(iter.next().unwrap(), num.parse::<u32>().unwrap()));
							}
							None => panic!(),
						}
					}

					return Command::LS(ls);
				}

				let first_line_parts = command_vector[0].split(" ").collect::<Vec<&str>>();

				if first_line_parts[0] == "cd" {
					let cd_command = match first_line_parts[1] {
						".." => ChangeDirectory::Up,
						"/" => ChangeDirectory::Root,
						path => ChangeDirectory::Down(path),
					};
					return Command::CD(cd_command);
				}

				panic!("Invalid command");
			})
			.collect();

		return commands;
	}

	pub fn apply_command(&mut self, command: Command<'a>) {
		match command {
			Command::CD(cd) => {
				self.fs.chage_dir(cd);
			}
			Command::LS(ls) => {
				for dir_name in ls.dirs {
					self.fs.make_dir(dir_name);
				}

				for file in ls.files {
					unsafe {
						self.fs.cwd.as_mut().unwrap().add_file(file);
					}
				}
			}
		}
	}
}
