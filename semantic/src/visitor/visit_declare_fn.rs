use crate::scope_stack::ScopeStack;
use crate::visitor::visit_block_statement::visit_block_statement;
use crate::visitor::visitor::print_info_extend;
pub use parse::ast_node::AstNode;
pub use parse::ast_node_type::AstNodeType;

/// 处理方法表达式
pub fn visit_fn_declare_stmt(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_info_extend("visit fn declare statement", ast_node);

	let (_arguments, _return_type, mut block_stmt) = ast_node.remove_fn_child();

	visit_block_statement(&mut block_stmt, scope_stack);
}
