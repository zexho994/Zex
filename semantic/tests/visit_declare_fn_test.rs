pub use parse::*;
pub use semantic::*;

#[test]
fn visit_fn_declare_1() {
	let str = String::from("fn foo () {int i = 10;echo i;}int i = 2;echo i;");
	println!("\n test => input : {}", str);
	let mut tokens = lexer::lexing(str);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}

#[test]
fn visit_fn_declare_2() {
	let str = String::from("fn foo () {int i = 5;echo i;}");
	println!("\n test => input : {}", str);
	let mut tokens = lexer::lexing(str);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}

#[test]
fn visit_fn_declare_3() {
	let str = String::from("int i = 1; fn foo () {echo i;}");
	println!("\n test => input : {}", str);
	let mut tokens = lexer::lexing(str);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}
