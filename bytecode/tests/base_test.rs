extern crate bytecode;

use bytecode::*;

#[test]
fn test_inti() {
	let str = "{int i = 1 + 1;}";
	let mut tokens = lexer::lexing(str.to_string());
	let ast = parse::parsing(&mut tokens).unwrap();
	to_bytecode(ast);
}
