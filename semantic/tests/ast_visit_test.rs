pub use parse::*;
pub use semantic::*;

#[test]
#[ignore]
fn repeat_declare() {
	let s = String::from("int a = 1;int a =2;");
	let mut tokens = lexer::lexing(s);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}

#[test]
fn visit_test() {
	let s = String::from(
		"
 {
	int a = 5 + 5;
	echo a;
 }

 { 
	int a = 20; 
	echo a;
 }

 int a = 30;
 echo a;

 a = 40;
 echo a;

 a = 30 + 20;
 echo a;

 int b = 2 + 2 * 3;
 echo b;

 b = 2 * 10 + 2 * 10;
 echo b;

 fn foo () {
	 int c = 30;
	 echo c;
 }
	",
	);
	println!("input : {}", s);
	let mut tokens = lexer::lexing(s);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}

#[test]
fn visit_fn_declare() {
	let str = String::from("fn foo () {int i = 1;echo i;}");
	println!("\n test => input : {}", str);
	let mut tokens = lexer::lexing(str);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}