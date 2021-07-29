extern crate bytecode;

use crate::file::create_file;
use crate::file::write_0x_to_file;
use crate::file::write_to_file;
use bytecode::file::append_to_file;
use bytecode::*;
use std::fs::read;

static PATH_STR: &'static str = "/Users/zexho/Github/Zex/sample/";

#[test]
fn write_test() {
	let path = PATH_STR.to_owned() + "write_test.class";
	let new_file = create_file(&path);
	write_to_file(new_file, "hello word");
	append_to_file(&path, "im zex");
}

#[test]
fn write_cafebabe() {
	// let magic = "cafebabe";
	let magic: u32 = 0xCAFEBABE;
	println!("{:b}", magic); // 11001010111111101011101010111110 == cafebabe
	let new_file = create_file(&(PATH_STR.to_owned() + "write_class.class"));

	write_0x_to_file(new_file, magic);
}

#[test]
fn read_class() {
	let content = read("/Users/zexho/Github/Zex/java/Simply.class").unwrap();
	println!("{:?}", content);
}

#[test]
fn read_cafebabe() {
	let content = read("/Users/zexho/Github/Zex/sample/write_class.class").unwrap();
	println!("{:?}", content);
}
