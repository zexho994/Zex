pub use parse::*;
pub use semantic::*;

/// 正确
#[test]
fn should_success() {
	let str = String::from("class A { int i = 1; echo i;}int i = 2;echo i;");
	println!("\n test => input : {}", str);
	let mut tokens = lexer::lexing(str);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}

/// 作用域外，i不生效
#[test]
#[ignore]
fn should_fail() {
	let str = String::from("class A { int i = 1;}  i = 2; echo i;");
	println!("\n test => input : {}", str);
	let mut tokens = lexer::lexing(str);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}
