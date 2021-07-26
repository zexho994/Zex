use crate::scope_stack::ScopeStack;
use crate::symbol::Symbol;
use crate::symbol::SYMBOL_TYPE_VARIABLE;
use crate::visitor::visitor::*;
pub use parse::ast_node::AstNode;
pub use parse::ast_node_type::AstNodeType;

/// ast_node type = AstNodeType::AssignmentStmt
/// 赋值语句将更新已有的变量值:
/// 1. 确保变量已经声明
/// 2. 更新变量的值
pub fn visit_assignment_stmt(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_info("visit var declare stmt");
	let var_id: String = ast_node.get_child_text(0).unwrap();
	let new_symbol = Symbol::new(
		var_id.clone(),
		SYMBOL_TYPE_VARIABLE,
		Option::Some(ast_node.remove_child(2)),
	);
	let mut current_scope = scope_stack.pop().unwrap();
	current_scope.update_symbol(&var_id, new_symbol, scope_stack);
	scope_stack.push(current_scope);
}
