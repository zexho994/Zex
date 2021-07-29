extern crate bytecode;

use crate::file::create_file;
use crate::file::write_to_file;
use bytecode::file::append_to_file;
use bytecode::*;

#[test]
fn write_test() {
	let path_str: &str = "/Users/zexho/Github/Zex/sample/test.class";
	let new_file = create_file(path_str);
	write_to_file(new_file, "hello word");
	append_to_file(path_str, "im zex");
}

#[test]
#[ignore]
fn format_test() {
	let magic: u32 = 0xCAFEBABE;
	println!("{:b}", magic); // 11001010111111101011101010111110 == cafebabe
}
