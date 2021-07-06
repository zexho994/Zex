pub use parse::*;
pub use semantic::*;

#[test]
fn visit_block_stmt() {
	let s = String::from("{ int a = 1 ; }");
	let mut tokens = lexer::lexing(s);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}

#[test]
fn repeat_declare() {
	let s = String::from("int a = 1; { int a = 1 ; }");
	let mut tokens = lexer::lexing(s);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}
