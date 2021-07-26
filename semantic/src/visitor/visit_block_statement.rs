use crate::scope::Scope;
use crate::scope_stack::ScopeStack;
use crate::visitor::visitor::print_info;
use crate::visitor::visitor::print_panic_more;
use crate::visitor::visitor::visit_statements;
pub use parse::ast_node_type::AstNodeType;

/// ast_node type = AstNodeType::BlockStmt
/// block域的父域是上一层域
pub fn visit_block_statement(
	ast_node: &mut parse::ast_node::AstNode,
	scope_stack: &mut ScopeStack,
) {
	print_info("visit block statement");
	let current_scope: &Scope = scope_stack.current_scope();
	let new_local_scope: Scope = Scope::new_local(current_scope.get_scope_name());
	scope_stack.push(new_local_scope);

	visit_block_statement_children(ast_node, scope_stack);

	scope_stack.pop();
}

pub fn visit_block_statement_children(
	ast_node: &mut parse::ast_node::AstNode,
	scope_stack: &mut ScopeStack,
) {
	for child in ast_node.get_child_vec_mut().iter_mut() {
		match child.get_type() {
			AstNodeType::Statements => visit_statements(child, scope_stack),
			_ => print_panic_more(
				"visit block statement children, child node type error",
				child,
			),
		}
	}
}
