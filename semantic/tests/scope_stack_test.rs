extern crate semantic;

use semantic::scope::*;
use semantic::scope_stack::*;
use semantic::semantic;

#[test]
fn scope_stack_push() {
	let mut stack = ScopeStack::new();
	let gs = Scope::new_global();
	stack.push(gs);
	let ls1 = Scope::new_local(stack.current_scope().get_scope_name());
	stack.push(ls1);
	let ls2 = Scope::new_local(stack.current_scope().get_scope_name());
	stack.push(ls2);
	stack.pop();
	let ls3 = Scope::new_local(stack.current_scope().get_scope_name());
	stack.push(ls3);

	assert!(stack.len() == 3);
}

#[test]
fn visit_block_statement() {
	let str = "{int i = 1 + 1;}";
	let mut tokens = lexer::lexing(str.to_string());
	let ast = parse::parsing(&mut tokens).unwrap();
	semantic(ast);
}
