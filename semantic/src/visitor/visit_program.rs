use crate::scope::Scope;
use crate::scope_stack::ScopeStack;
use crate::visitor::visitor::print_info;
use crate::visitor::visitor::visit_statements;
pub use parse::ast_node::AstNode;

// 1. 创建全局域
// 2. 处理流程
// 3. 保存，退出
pub fn visit_program(ast_node: &mut AstNode) {
	print_info("visit program");
	// 初始化全局域
	let global_scope = Scope::new_global();
	// 初始化域栈
	let mut scope_stack = ScopeStack::new();
	// 入栈
	scope_stack.push(global_scope);

	// 访问子节点
	visit_program_children(ast_node, &mut scope_stack);

	//出栈
	scope_stack.pop();
}

/// visit program的所有子节点
fn visit_program_children(program_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	for child in program_node.get_child_vec_mut().iter_mut() {
		visit_statements(child, scope_stack);
	}
}
