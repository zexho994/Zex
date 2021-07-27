pub use parse::*;
pub use semantic::*;

#[test]
// #[ignore]
/// 变量重复声明
fn variable_reap_declare() {
	let str = String::from("int a;int a;");
	println!("\n test => input : {}", str);
	let mut tokens = lexer::lexing(str);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}

#[test]
// #[ignore]
/// 变量未初始化
fn variable_uninitialized() {
	let str = String::from("int a;echo a;");
	println!("\n test => input : {}", str);
	let mut tokens = lexer::lexing(str);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}

#[test]
// #[ignore]
/// 变量未声明
fn variable_undefined() {
	let str = String::from("{int a;}a = 1;");
	println!("\n test => input : {}", str);
	let mut tokens = lexer::lexing(str);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}
