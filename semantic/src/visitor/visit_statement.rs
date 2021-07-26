use crate::scope_stack::ScopeStack;
use crate::visitor::visit_declare_fn::visit_fn_declare_stmt;
use crate::visitor::visit_declare_variable::visit_var_declare_stmt;
use crate::visitor::visit_statement_assignment::visit_assignment_stmt;
use crate::visitor::visitor::print_info;
use crate::visitor::visitor::print_panic;
use crate::visitor::visitor::print_panic_more;
use crate::visitor::visitor::visit_echo;
pub use parse::ast_node::AstNode;
pub use parse::ast_node_type::AstNodeType;

/// ast_node type = AstNodeType::Statement
///
/// statement 的类型
/// 1. echo 语句
/// 2. varDeclareStmt 声明语句
/// 3. assignmentStmt 赋值语句
/// 4. expressionStmt 表达式语句
pub fn visit_statement(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_info("visit statement");

	// visit children
	for child in ast_node.get_child_vec_mut().iter_mut() {
		match child.get_type() {
			AstNodeType::Echo => visit_echo(child, scope_stack),
			AstNodeType::VarDeclareStmt => visit_var_declare_stmt(child, scope_stack),
			AstNodeType::FnDeclareStmt => visit_fn_declare_stmt(child, scope_stack),
			AstNodeType::AssignmentStmt => visit_assignment_stmt(child, scope_stack),
			AstNodeType::ExpressionStmt => print_panic("visit expr error"),
			_ => print_panic_more("visit statement child error", child),
		}
	}
}
