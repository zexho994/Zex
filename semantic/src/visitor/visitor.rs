use crate::ast_node::AstNode;
use crate::ast_node_type::AstNodeType;
use crate::scope_stack::ScopeStack;
use crate::symbol::*;

/// <echo> ::= echo (<id> |<intLiteral>),
/// 获取要输出的节点，计算值
/// 要输出的值必需在合法作用域中，且在合法生命周期内
/// 如果要输出的字面值类型，直接打印即可
/// 如果要输出的是其他类型，需要进行计算,计算规则:
/// -
/// - Identifier: 从本域开始向超查找，找到对应变量，然后进行求值计算
/// - ExpressionStmt: todo
/// -
///
/// echo 3;     //intLiteral
/// int a = 1;	// a:primary
/// echo a;	    //identifier a
/// int b = a + 2;  //b:expressionStmt
/// echo b;     //identifier b
///
pub fn visit_echo(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_info_extend("visit echo", &ast_node);
	let target = ast_node.get_child_mut(0).unwrap();
	match target.get_type() {
		// 标识符类型，需要先获取id对应的symbol，然在再获取对应的AstNode
		AstNodeType::Identifier => visit_identifier(target, scope_stack),
		// 字面量类型，可以直接输出
		AstNodeType::IntLiteral => echo_int_literal(target),
		_ => panic!("visit echo"),
	}
}

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

pub fn echo_int_literal(ast_node: &mut AstNode) {
	println!("{}", ast_node.get_text().clone())
}

pub fn print_info(_msg: &str) {
	// println!("[info][ast_visit]: {}", _msg);
}

pub fn print_info_extend<T: std::fmt::Debug>(_msg: &str, _t: &T) {
	// println!("[info][ast_visit]: {},t = {:?}", _msg, _t);
}

pub fn print_panic(msg: &str) {
	panic!("[error][ast_visit] {}", msg);
}

pub fn print_panic_more<T: std::fmt::Debug>(msg: &str, t: &T) {
	panic!("[error][visitor] msg = {}, t = {:?}", msg, t);
}
