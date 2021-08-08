// module to handle some file actions

use std::{
	fs::File,
	io::{BufRead, BufReader},
	path::Path,
};

// create a file at 'given_path'
pub fn create_file(given_path: String) -> File {
	let file_path = Path::new(&given_path);

	return match File::create(&file_path) {
		Ok(file) => file,
		Err(why) => panic!("could not create {}: {}", file_path.display(), why),
	};
}

// open a file at 'given_path'
pub fn open_file(given_path: String) -> File {
	let file_path = Path::new(&given_path);

	return match File::open(&file_path) {
		Ok(file) => file,
		Err(why) => panic!("could not open {}: {}", file_path.display(), why),
	};
}

// read indiviual lines of a given file into a vector
pub fn read_lines(file: File) -> Vec<String> {
	let mut lines: Vec<String> = Vec::new();

	let file_reader = BufReader::new(file);
	for line in file_reader.lines() {
		lines.push(line.unwrap());
	}

	return lines;
}
