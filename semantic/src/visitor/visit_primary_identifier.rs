use crate::scope_stack::ScopeStack;
use crate::symbol::Symbol;
use crate::visitor::visitor::print_info_extend;
use parse::ast_node::AstNode;

/// ### identifier可能的场景
/// 符号表中:k=id,v=ast_node
/// ast_node's type = expressionStmt
/// 根据id的text在符号表从下到上遍历寻找对应符号以及值
pub fn visit_identifier(ast_node: &mut AstNode, scope_stack: &ScopeStack) {
	print_info_extend("visit identifier", ast_node);
	let id = ast_node.get_text();
	let mut target_symbol: Option<&Symbol> = None;

	// 在当前域中查找符号
	if let Some(symbol) = scope_stack.current_scope().find_symbol(&id) {
		target_symbol = Some(symbol);
	} else {
		let mut parent_name = scope_stack.current_scope().parent_scope_name();
		//父域中查找符号
		while parent_name.is_some() {
			let scope = scope_stack.find_scope(&parent_name.unwrap()).unwrap();
			if let Some(symbol) = scope.find_symbol(&id) {
				target_symbol = Some(symbol);
				break;
			}
			parent_name = scope.parent_scope_name();
		}
	}
	let ast_node = target_symbol.unwrap().get_ast_node().unwrap();
	let num = AstNode::calculate(ast_node);
	println!("{}", num);
}
