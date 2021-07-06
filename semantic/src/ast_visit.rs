use super::ast_node::AstNode;
use super::ast_node_type::AstNodeType;
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

	for child in ast_node._child.iter_mut() {
		visit_statements(child, &mut scope_stack);
	}

	// 退出全局域
	scope_stack.pop();
}

fn visit_statements(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_visit_info("visit statements");

	for child in ast_node._child.iter_mut() {
		match child._type {
			AstNodeType::BlockStmt => {
				visit_block_statement(child, scope_stack);
			}
			AstNodeType::Statement => {
				visit_statements(child, scope_stack);
			}
			_ => {}
		}
	}
}

/// block域的父域是上一层域
fn visit_block_statement(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_visit_info("visit block statement");
	let pre_scope = scope_stack.current().unwrap();
	let block_scope: Scope = Scope::new_local(pre_scope.scope_name.clone());
	scope_stack.push(block_scope);

	visit_statements(ast_node, scope_stack);

	scope_stack.pop();
}

fn visit_statement(ast_node: &mut AstNode) {
	print_visit_info("visit statement");
}

fn visit_assignment(ast_node: &mut AstNode, scope_stack: &ScopeStack) {}

fn print_visit_info(msg: &str) {
	println!("[info][ast_visit]: {}", msg);
}
