extern crate parse;

pub use parse::*;

#[test]
fn parse_block_stmt() {
    let s = String::from("{ int a = 1 + 3 * 2;{int b = 1;} }");
    println!("\n==> parse str {}", s);
    let mut tokens = lexer::lexing(s);
    let ast = parsing(&mut tokens);
    println!("\n==> parse str to ast , ast is {:?}", ast);
}

#[test]
fn parse_assignment() {
    let s = String::from("int a;a=1;echo a;");
    println!("\n===> parse assignment {}", s);
    let mut tokens = lexer::lexing(s.clone());
    let ast = parsing(&mut tokens);
    println!("\n==> parse str to ast , ast is {:?}", ast);
}

#[test]
fn parse_echo() {
    let s = String::from("echo 1;echo a;");
    println!("\n===> parse assignment {}", s);
    let mut tokens = lexer::lexing(s.clone());
    let ast = parsing(&mut tokens);
    println!("\n==> parse str to ast , ast is {:?}", ast);
}

#[test]
fn parse_fn_declare() {
    let s = String::from("fn foo () {int a; }");
    println!("\n===> parse str {}", s);
    let mut tokens = lexer::lexing(s.clone());
    let ast = parsing(&mut tokens);
    println!("\n==> parse str to ast , ast is {:?}", ast);
}

#[test]
fn parse_class_declare() {
    let s = String::from("class Zex { int i = 1;}");
    println!("\n===> parse str {}", s);
    let mut tokens = lexer::lexing(s.clone());
    let ast = parsing(&mut tokens);
    println!("\n==> parse str to ast , ast is {:?}", ast);
}
