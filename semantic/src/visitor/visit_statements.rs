use crate::scope_stack::ScopeStack;
use crate::visitor::visit_block_statement::visit_block_statement;
use crate::visitor::visitor::print_info;
use crate::visitor::visitor::print_panic_more;
use crate::visitor::visitor::visit_statement;
pub use parse::ast_node::AstNode;
pub use parse::ast_node_type::AstNodeType;

/// ast_node type = AstNodeType::Statements
pub fn visit_statements(ast_node: &mut parse::ast_node::AstNode, scope_stack: &mut ScopeStack) {
	print_info("visit statements");
	visit_statements_children(ast_node, scope_stack)
}

fn visit_statements_children(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	for child in ast_node.get_child_vec_mut().iter_mut() {
		match child.get_type() {
			AstNodeType::BlockStmt => {
				visit_block_statement(child, scope_stack);
			}
			AstNodeType::Statement => {
				visit_statement(child, scope_stack);
			}
			_ => print_panic_more("visit statements children, child node type error", child),
		}
	}
}
