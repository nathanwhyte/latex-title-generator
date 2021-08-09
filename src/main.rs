use std::io;

mod files;

struct FileMetadata {
	path: String,
	class: String,
	author: String,
	title: String,
}

fn main() {
	let file_data: FileMetadata = collect_user_input();

	println!("path: {}", file_data.path);
	println!("class: {}", file_data.class);
	println!("author: {}", file_data.author);
	println!("title: {}", file_data.title);
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

	return data;
}
