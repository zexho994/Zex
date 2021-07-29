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

pub fn write_0x_to_file(mut f: File, x: u32) {
	let b1: u8 = ((x >> 24) & 0xff) as u8;
	let b2: u8 = ((x >> 16) & 0xff) as u8;
	let b3: u8 = ((x >> 8) & 0xff) as u8;
	let b4: u8 = (x & 0xff) as u8;

	let data: [u8; 4] = [b1, b2, b3, b4];
	f.write(&data).unwrap();
}
