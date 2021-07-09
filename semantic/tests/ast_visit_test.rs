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
/// { int a = 1 ;}  //success
/// { int a = 1 ;}  //success
/// int a = 1;      //success
/// 上面三个a都能声明成功
///
/// int a = 1;    //success
/// {int a = 1;}  // failure. 因为a已经声明
///
#[test]
fn visit_test() {
	let s = String::from(
		"{
			int a = 1;
			echo a;
		 }
	     { 
		    int a = 1; 
		 }
		 int a = 1;
		 echo a;
		 a = 2;
         echo a;
		 a = 1 + 2;
		 echo a;
		 int b = 2 + 2 * 3;
		 echo b;
		 b = 2 * 10 + 2 * 10;
		 echo b;
		",
	);
	let mut tokens = lexer::lexing(s);
	let ast = parsing(&mut tokens).unwrap();
	semantic(ast);
}
