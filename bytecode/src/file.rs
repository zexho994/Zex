use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn create_file(path_str: &str) -> File {
	File::create(path_str).unwrap()
}

pub fn write_to_file(mut f: File, data: &str) {
	f.write(data.as_bytes()).unwrap();
}

pub fn append_to_file(file_path: &str, data: &str) {
	let mut f = OpenOptions::new().append(true).open(file_path).unwrap();
	f.write(data.as_bytes()).unwrap();
}
