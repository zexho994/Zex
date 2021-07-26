use crate::scope_stack::ScopeStack;
use crate::visitor::visit_primary_identifier::visit_identifier;
use crate::visitor::visitor::echo_int_literal;
use crate::visitor::visitor::print_info_extend;
pub use parse::ast_node::AstNode;
pub use parse::ast_node_type::AstNodeType;

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
