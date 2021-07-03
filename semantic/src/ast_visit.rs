use super::ast_node::AstNode;
use super::scope::Scope;
use super::scope::ScopeStack;

// 1. 创建全局域
// 2. 处理流程
// 3. 保存，退出
pub fn visit_program(ast_node: &mut AstNode) {
	print_visit_info("visit program");
	// 创建作用域栈
	let mut scope_stack = ScopeStack::new();
	// 创建全局域
	let global_scope = Scope::new_global();
	scope_stack.push(global_scope);

	visit_statements(ast_node, &mut scope_stack);

	// 退出全局域
	scope_stack.pop();
}

fn visit_statements(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_visit_info("visit statements");
}

fn visit_block_statement(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_visit_info("visit block statement");
	let mut block_scope = Scope::new_local();
	scope_stack.push(block_scope);


}

fn visit_statement(ast_node: &mut AstNode) {}

fn visit_assignment(ast_node: &mut AstNode, scope_stack: &ScopeStack) {}

fn print_visit_info(msg: &str) {
	println!("[info][ast_visit]: {}", msg);
}
