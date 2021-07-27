use crate::scope_stack::ScopeStack;
use crate::visitor::visit_block_statement::visit_block_statement;
use crate::visitor::visitor::print_info_extend;
pub use parse::ast_node::AstNode;

/// 处理方法表达式
pub fn visit_class_declare_stmt(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_info_extend("visit fn declare statement", ast_node);

	let mut block_stmt = ast_node.remove_class_child();

	visit_block_statement(&mut block_stmt, scope_stack);
}
