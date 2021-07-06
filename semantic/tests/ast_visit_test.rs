pub use parse::*;
pub use semantic::*;

#[test]
fn parse_block_stmt() {
	let s = String::from("{ int a = 1 ; }");
	let mut tokens = lexer::lexing(s);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}
