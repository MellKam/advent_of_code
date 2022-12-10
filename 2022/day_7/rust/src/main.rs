mod file_system;
mod parser;

use file_system::FileSystem;
use parser::FileSystemParser;

fn main() {
	let mut fs_parser = FileSystemParser::new(FileSystem::new(70_000_000));

	let commands = fs_parser.parse_commands_string(include_str!("../../input.txt"));

	for command in commands {
		fs_parser.apply_command(command);
	}

	let target_unused_space: u32 = 30_000_000;
	let target_free_space = target_unused_space - fs_parser.fs.get_unused_space();

	println!(
		"{:?}",
		fs_parser
			.fs
			.get_smallest_folder_to_delete(target_free_space)
	);
}
