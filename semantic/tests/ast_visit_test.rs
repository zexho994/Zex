pub use parse::*;
pub use semantic::*;

#[test]
#[ignore]
fn repeat_declare() {
	let s = String::from("int a = 1;{ int a = 1 ; }");
	let mut tokens = lexer::lexing(s);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}

/// 变量重复声明测试
/// { int a = 1 ;}
/// { int a = 1 ;}
/// int a = 1;
/// 上面三个a都能声明成功
///
/// int a = 1;
/// {int a = 1;}  // failure,a已经声明
///
#[test]
fn visit_block_stmt() {
	let s = String::from("{int a = 1;} { int a = 1 ; } int a = 1;");
	let mut tokens = lexer::lexing(s);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}
