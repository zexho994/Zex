use super::ast_node::AstNode;
use super::ast_node_type::AstNodeType;
use super::scope::Scope;
use super::scope::ScopeStack;

// 1. 创建全局域
// 2. 处理流程
// 3. 保存，退出
pub fn visit_program(ast_node: &mut AstNode) {
	print_visit_info("visit program");
	// 创建作用域栈
	let mut scope_stack = ScopeStack::new();
	// 创建全局域
	let global_scope = Scope::new_global();
	scope_stack.push(global_scope);

	for child in ast_node._child.iter_mut() {
		visit_statements(child, &mut scope_stack);
	}

	// 退出全局域
	scope_stack.pop();
}

fn visit_statements(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_visit_info("visit statements");

	for child in ast_node._child.iter_mut() {
		match child._type {
			AstNodeType::BlockStmt => {
				visit_block_statement(child, scope_stack);
			}
			AstNodeType::Statement => {
				visit_statement(child, scope_stack);
			}
			_ => {}
		}
	}
}

/// block域的父域是上一层域
fn visit_block_statement(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_visit_info("visit block statement");
	let pre_scope = scope_stack.current().unwrap();
	let block_scope: Scope = Scope::new_local(pre_scope.scope_name.clone());
	scope_stack.push(block_scope);

	visit_statements(ast_node, scope_stack);

	scope_stack.pop();
}

fn visit_statement(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_visit_info("visit statement");

	for child in ast_node._child.iter_mut() {
		match child._type {
			AstNodeType::AssignmentStmt => {}
			AstNodeType::VarDeclareStmt => {
				visit_var_declare_stmt(child, scope_stack);
			}
			AstNodeType::ExpressionStmt => {}
			_ => panic!("visit statement error"),
		}
	}
}

/// ### varDeclareStmt结构
/// type, id, assignment, expressionStm
///
/// ## 变量声明语句检查规则
/// 1. 变量的作用域存在本scope中
/// 2. 合法scope中不能有重名变量
///    2.1. 本scope中不能有
///    2.2. 递归向上的所有scope中都不能有
///
/// ```text
/// int a = 1;
/// {
/// 	int a = 2;  //error,上级域中已经有a
/// 	int b = 2;  
/// }
/// int b = 3;   //success,块中的b已经失效了
/// ```
fn visit_var_declare_stmt(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_visit_info("visit var declare stmt");
	let current_scope = scope_stack.current().unwrap();
	let var_type = ast_node.get_child(0).unwrap();
	let var_id = ast_node.get_child_text(1).unwrap();
	let var_assign = ast_node.get_child(2);
	let var_exprStm = ast_node.get_child(3);

	// 检查本scope
	if current_scope.has_variable(var_id.clone()) {}

	// 检查上级scope
}

fn print_visit_info(msg: &str) {
	println!("[info][ast_visit]: {}", msg);
}
