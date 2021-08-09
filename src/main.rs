use std::{fs::File, io};

mod files;

struct FileMetadata {
	path: String,
	class: String,
	author: String,
	title: String,
}

fn main() {
	let file_data: FileMetadata = collect_user_input();

	put_file_metadata(file_data);

	// println!("path: {}", file_data.path);
	// println!("class: {}", file_data.class);
	// println!("author: {}", file_data.author);
	// println!("title: {}", file_data.title);
}

// function to facilitate collecting .tex file metadata
fn collect_user_input() -> FileMetadata {
	return FileMetadata {
		path: get_file_data(String::from("path")),
		class: get_file_data(String::from("class")),
		author: get_file_data(String::from("author")),
		title: get_file_data(String::from("title")),
	};
}

// general function to collect a peice of .tex file metadata
fn get_file_data(what_data: String) -> String {
	println!("please enter the file's {}: ", what_data);

	let mut data = String::new();
	io::stdin()
		.read_line(&mut data)
		.expect("error reading input");

	println!("file {}: {}\n", what_data, data);

	return data.replace("\n", "");
}

fn put_file_metadata(file_data: FileMetadata) {
	create_tex_file(file_data.path, file_data.title);
}

fn create_tex_file(path: String, title: String) -> File {
	let mut path_copy = path.clone();

	if count_chars(path_copy.clone(), '/') > 1 || path_copy.chars().count() > 2 {
		path_copy.push('/');
	}

	let file_name_lower = title.to_lowercase().clone();
	let mut file_name_in_path = file_name_lower.replacen(" ", "-", count_chars(title, ' '));

	println!("path: {}", path_copy);

	file_name_in_path.push_str(".tex");
	path_copy.push_str(&file_name_in_path);

	println!("full path: {}", &path_copy);

	// TODO add an extra '/' to path_copy before file_name_lower
	// TODO add '.tex' to the end of path_copy

	return files::create_file(&path_copy);
}

fn count_chars(check_string: String, target_char: char) -> usize {
	let mut count: usize = 0;
	for character in check_string.chars() {
		if character == target_char {
			count += 1;
		}
	}
	return count;
}
