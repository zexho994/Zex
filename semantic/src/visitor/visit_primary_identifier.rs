use crate::scope::Scope;
use crate::scope_stack::ScopeStack;
use crate::visitor::visitor::print_info_extend;
use crate::visitor::visitor::print_panic_more;
use parse::ast_node::AstNode;

/// ### identifier可能的场景
/// 符号表中:k=id,v=ast_node
/// ast_node's type = expressionStmt
/// 根据id的text在符号表从下到上遍历寻找对应符号以及值
pub fn visit_identifier(ast_node: &mut AstNode, scope_stack: &ScopeStack) {
	print_info_extend("visit identifier", ast_node);

	if let Some(symbol) = Scope::loop_find_symbol(ast_node.get_text().clone(), scope_stack) {
		if let Some(node) = symbol.get_ast_node() {
			let num = AstNode::calculate(node);
			println!("{}", num);
		} else {
			print_panic_more("变量未初始化", ast_node)
		}
	} else {
		print_panic_more("变量未声明", ast_node)
	}
}
