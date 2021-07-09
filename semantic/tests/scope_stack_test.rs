extern crate semantic;

use semantic::scope::*;
use semantic::scope_stack::*;
use semantic::semantic;

#[test]
fn scope_stack_push() {
	let mut stack = ScopeStack::new();
	let gs = Scope::new_global();
	stack.push(gs);
	let ls1 = Scope::new_local(stack.current_scope().unwrap().scope_name.clone());
	stack.push(ls1);
	let ls2 = Scope::new_local(stack.current_scope().unwrap().scope_name.clone());
	stack.push(ls2);
	stack.pop();
	let ls3 = Scope::new_local(stack.current_scope().unwrap().scope_name.clone());
	stack.push(ls3);

	assert!(stack.stack.len() == 3);
	match stack.current_scope() {
		Some(e) => {
			println!("current e is {:?}", e);
		}
		None => panic!("current获取失败"),
	}
}

#[test]
fn visit_block_statement() {
	let str = "{int i = 1 + 1;}";
	let mut tokens = lexer::lexing(str.to_string());
	let ast = parse::parsing(&mut tokens).unwrap();
	semantic(ast);
}
