use crate::scope_stack::ScopeStack;
use crate::symbol::Symbol;
use crate::symbol::SYMBOL_TYPE_VARIABLE;
use crate::symbol::UN_DEFINED;
use crate::visitor::visitor::print_info;
pub use parse::ast_node::AstNode;
pub use parse::ast_node_type::AstNodeType;

/// <expressionStm> ::= <addExpr>
/// <addExpr> ::= <mulExpr> | <mulExpr> '+' <addExpr>
/// <mulExpr> ::= <primary> | <primary> '*' <mulExpr>
/// <primary> :: <primary> ::= <id> | <intLiteral>
/// fn visit_expression_stmt(child: &mut AstNode, scope_stack: &mut ScopeStack) {}
///
/// ### varDeclareStmt结构
/// type, id, assignment, expressionStm
///
/// ## 变量声明语句检查规则
/// 1. 变量的作用域存在本scope中
/// 2. 合法scope中不能有重名变量
///    2.1. 本scope中不能有
///    2.2. 递归向上的所有scope中都不能有
/// 3. 保存变量的值的AstNode
///
/// ```text
/// int a = 1;
/// {
/// 	int a = 2;  //error,上级域中已经有a
/// 	int b = 2;
/// }
/// int b = 3;   //success,块中的b已经失效了
/// ```
pub fn visit_var_declare_stmt(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_info("visit var declare stmt");
	if ast_node.child_count() < 4 {
		declare_not_val(ast_node, scope_stack);
	} else {
		declare_with_val(ast_node, scope_stack);
	}
}

/// 声明，有赋值
fn declare_with_val(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	let var_id: String = ast_node.get_child_text(1).unwrap();
	let mut scope = scope_stack.pop().unwrap();

	let ast_node = ast_node.remove_child(3);
	let symbol = Symbol::new(var_id, SYMBOL_TYPE_VARIABLE, Option::Some(ast_node));

	scope.define_symbol(symbol, scope_stack);
	scope_stack.push(scope);
}

/// 声明，没有赋值
fn declare_not_val(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	let var_id: String = ast_node.get_child_text(1).unwrap();
	let mut scope = scope_stack.pop().unwrap();

	let symbol = Symbol::new(var_id, UN_DEFINED, None);

	scope.define_symbol(symbol, scope_stack);
	scope_stack.push(scope);
}
